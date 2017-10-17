// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_upper_case_globals)]

use libc;
use std::ptr;
pub use base::{CGError, boolean_t};
pub use geometry::{CGRect, CGPoint, CGSize};

use core_foundation::base::{CFRetain, TCFType};
use image::CGImage;
use foreign_types::ForeignType;

pub type CGDirectDisplayID = libc::uint32_t;
pub type CGWindowID        = libc::uint32_t;

pub const kCGNullWindowID: CGWindowID = 0 as CGWindowID;


pub type CGWindowListOption = libc::uint32_t;

pub const kCGWindowListOptionAll:              CGWindowListOption    = 0;
pub const kCGWindowListOptionOnScreenOnly:     CGWindowListOption    = 1 << 0;
pub const kCGWindowListOptionOnScreenAboveWindow: CGWindowListOption = 1 << 1;
pub const kCGWindowListOptionOnScreenBelowWindow: CGWindowListOption = 1 << 2;
pub const kCGWindowListOptionIncludingWindow:  CGWindowListOption    = 1 << 3;
pub const kCGWindowListExcludeDesktopElements: CGWindowListOption    = 1 << 4;

pub type CGWindowImageOption = libc::uint32_t;

pub const kCGWindowImageDefault: CGWindowImageOption = 0;
pub const kCGWindowImageBoundsIgnoreFraming: CGWindowImageOption = 1 << 0;
pub const kCGWindowImageShouldBeOpaque: CGWindowImageOption = 1 << 1;
pub const kCGWindowImageOnlyShadows: CGWindowImageOption = 1 << 2;
pub const kCGWindowImageBestResolution: CGWindowImageOption = 1 << 3;
pub const kCGWindowImageNominalResolution: CGWindowImageOption = 1 << 4;

pub use core_foundation::dictionary::{ CFDictionary, CFDictionaryRef, CFDictionaryGetValueIfPresent };
pub use core_foundation::array::{ CFArray, CFArrayRef };
pub use core_foundation::array::{ CFArrayGetCount, CFArrayGetValueAtIndex };
pub use core_foundation::base::{  CFIndex, CFRelease, CFTypeRef };

#[derive(Copy, Clone, Debug)]
pub struct CGDisplay {
    pub id: CGDirectDisplayID,
}

foreign_type! {
    #[doc(hidden)]
    type CType = ::sys::CGDisplayMode;
    fn drop = CGDisplayModeRelease;
    fn clone = |p| CFRetain(p as *const _) as *mut _;
    pub struct CGDisplayMode;
    pub struct CGDisplayModeRef;
}

impl CGDisplay {
    #[inline]
    pub fn new(id: CGDirectDisplayID) -> CGDisplay {
        CGDisplay { id: id }
    }

    /// Returns the the main display.
    #[inline]
    pub fn main() -> CGDisplay {
        CGDisplay::new(unsafe { CGMainDisplayID() })
    }

    /// Returns the bounds of a display in the global display coordinate space.
    #[inline]
    pub fn bounds(&self) -> CGRect {
        unsafe { CGDisplayBounds(self.id) }
    }

    /// Returns information about a display's current configuration.
    #[inline]
    pub fn display_mode(&self) -> Option<CGDisplayMode> {
        unsafe {
            let mode_ref = CGDisplayCopyDisplayMode(self.id);
            if !mode_ref.is_null() {
                Some(CGDisplayMode::from_ptr(mode_ref))
            } else {
                None
            }
        }
    }

    /// Returns an image containing the contents of the specified display.
    #[inline]
    pub fn image(&self) -> Option<CGImage> {
        unsafe {
            let image_ref = CGDisplayCreateImage(self.id);
            if !image_ref.is_null() {
                Some(CGImage::from_ptr(image_ref))
            } else {
                None
            }
        }
    }

    /// Returns a composite image based on a dynamically generated list of
    /// windows.
    #[inline]
    pub fn screenshot(
        bounds: CGRect,
        list_option: CGWindowListOption,
        window_id: CGWindowID,
        image_option: CGWindowImageOption,
    ) -> Option<CGImage> {
        unsafe {
            let image_ref = CGWindowListCreateImage(bounds, list_option, window_id, image_option);
            if !image_ref.is_null() {
                Some(CGImage::from_ptr(image_ref))
            } else {
                None
            }
        }
    }

    /// Returns a composite image of the specified windows.
    #[inline]
    pub fn screenshot_from_windows(
        bounds: CGRect,
        windows: CFArray,
        image_option: CGWindowImageOption,
    ) -> Option<CGImage> {
        unsafe {
            let image_ref = CGWindowListCreateImageFromArray(
                bounds,
                windows.as_concrete_TypeRef(),
                image_option,
            );
            if !image_ref.is_null() {
                Some(CGImage::from_ptr(image_ref))
            } else {
                None
            }
        }
    }

    /// Generates and returns information about the selected windows in the
    /// current user session.
    pub fn window_list_info(
        option: CGWindowListOption,
        relative_to_window: Option<CGWindowID>,
    ) -> Option<CFArray> {
        let relative_to_window = relative_to_window.unwrap_or(kCGNullWindowID);
        let array_ref = unsafe { CGWindowListCopyWindowInfo(option, relative_to_window) };
        if array_ref != ptr::null() {
            Some(unsafe { TCFType::wrap_under_create_rule(array_ref) })
        } else {
            None
        }
    }

    /// Returns a Boolean value indicating whether a display is active.
    #[inline]
    pub fn is_active(&self) -> bool {
        unsafe { CGDisplayIsActive(self.id) != 0 }
    }

    /// Returns a boolean indicating whether a display is always in a
    /// mirroring set.
    #[inline]
    pub fn is_always_in_mirror_set(&self) -> bool {
        unsafe { CGDisplayIsAlwaysInMirrorSet(self.id) != 0 }
    }

    /// Returns a boolean indicating whether a display is sleeping (and is
    /// therefore not drawable.)
    #[inline]
    pub fn is_asleep(&self) -> bool {
        unsafe { CGDisplayIsAsleep(self.id) != 0 }
    }

    /// Returns a boolean indicating whether a display is built-in, such as
    /// the internal display in portable systems.
    #[inline]
    pub fn is_builtin(&self) -> bool {
        unsafe { CGDisplayIsBuiltin(self.id) != 0 }
    }

    /// Returns a boolean indicating whether a display is in a hardware
    /// mirroring set.
    #[inline]
    pub fn is_in_hw_mirror_set(&self) -> bool {
        unsafe { CGDisplayIsInHWMirrorSet(self.id) != 0 }
    }

    /// Returns a boolean indicating whether a display is in a mirroring set.
    #[inline]
    pub fn is_in_mirror_set(&self) -> bool {
        unsafe { CGDisplayIsInMirrorSet(self.id) != 0 }
    }

    /// Returns a boolean indicating whether a display is the main display.
    #[inline]
    pub fn is_main(&self) -> bool {
        unsafe { CGDisplayIsMain(self.id) != 0 }
    }

    /// Returns a boolean indicating whether a display is connected or online.
    #[inline]
    pub fn is_online(&self) -> bool {
        unsafe { CGDisplayIsOnline(self.id) != 0 }
    }

    /// Returns a boolean indicating whether Quartz is using OpenGL-based
    /// window acceleration (Quartz Extreme) to render in a display.
    #[inline]
    pub fn uses_open_gl_acceleration(&self) -> bool {
        unsafe { CGDisplayUsesOpenGLAcceleration(self.id) != 0 }
    }

    /// Returns a boolean indicating whether a display is running in a stereo
    /// graphics mode.
    #[inline]
    pub fn is_stereo(&self) -> bool {
        unsafe { CGDisplayIsStereo(self.id) != 0 }
    }

    /// For a secondary display in a mirroring set, returns the primary
    /// display.
    #[inline]
    pub fn mirrors_display(&self) -> CGDirectDisplayID {
        unsafe { CGDisplayMirrorsDisplay(self.id) }
    }

    /// Returns the primary display in a hardware mirroring set.
    #[inline]
    pub fn primary_display(&self) -> CGDirectDisplayID {
        unsafe { CGDisplayPrimaryDisplay(self.id) }
    }

    /// Returns the rotation angle of a display in degrees.
    #[inline]
    pub fn rotation(&self) -> f64 {
        unsafe { CGDisplayRotation(self.id) }
    }

    /// Returns the width and height of a display in millimeters.
    #[inline]
    pub fn screen_size(&self) -> CGSize {
        unsafe { CGDisplayScreenSize(self.id) }
    }

    /// Returns the serial number of a display monitor.
    #[inline]
    pub fn serial_number(&self) -> u32 {
        unsafe { CGDisplaySerialNumber(self.id) }
    }

    /// Returns the logical unit number of a display.
    #[inline]
    pub fn unit_number(&self) -> u32 {
        unsafe { CGDisplayUnitNumber(self.id) }
    }

    /// Returns the vendor number of the specified display's monitor.
    #[inline]
    pub fn vendor_number(&self) -> u32 {
        unsafe { CGDisplayVendorNumber(self.id) }
    }

    /// Returns the model number of a display monitor.
    #[inline]
    pub fn model_number(&self) -> u32 {
        unsafe { CGDisplayModelNumber(self.id) }
    }

    /// Returns the display height in pixel units.
    #[inline]
    pub fn pixels_high(&self) -> u64 {
        unsafe { CGDisplayPixelsHigh(self.id) as u64 }
    }

    /// Returns the display width in pixel units.
    #[inline]
    pub fn pixels_wide(&self) -> u64 {
        unsafe { CGDisplayPixelsWide(self.id) as u64 }
    }

    /// Provides a list of displays that are active (or drawable).
    #[inline]
    pub fn active_displays() -> Result<Vec<CGDirectDisplayID>, CGError> {
        let count = try!(CGDisplay::active_display_count());
        let mut buf: Vec<CGDirectDisplayID> = vec![0; count as usize];
        let result =
            unsafe { CGGetActiveDisplayList(count as u32, buf.as_mut_ptr(), ptr::null_mut()) };
        if result == 0 {
            Ok(buf)
        } else {
            Err(result)
        }
    }

    /// Provides count of displays that are active (or drawable).
    #[inline]
    pub fn active_display_count() -> Result<u32, CGError> {
        let mut count: libc::uint32_t = 0;
        let result = unsafe { CGGetActiveDisplayList(0, ptr::null_mut(), &mut count) };
        if result == 0 {
            Ok(count as u32)
        } else {
            Err(result)
        }
    }

    /// Hides the mouse cursor, and increments the hide cursor count.
    #[inline]
    pub fn hide_cursor(&self) -> Result<(), CGError> {
        let result = unsafe { CGDisplayHideCursor(self.id) };
        if result == 0 {
            Ok(())
        } else {
            Err(result)
        }
    }

    /// Decrements the hide cursor count, and shows the mouse cursor if the
    /// count is 0.
    #[inline]
    pub fn show_cursor(&self) -> Result<(), CGError> {
        let result = unsafe { CGDisplayShowCursor(self.id) };
        if result == 0 {
            Ok(())
        } else {
            Err(result)
        }
    }

    /// Moves the mouse cursor to a specified point relative to the display
    /// origin (the upper-left corner of the display).
    #[inline]
    pub fn move_cursor_to_point(&self, point: CGPoint) -> Result<(), CGError> {
        let result = unsafe { CGDisplayMoveCursorToPoint(self.id, point) };
        if result == 0 {
            Ok(())
        } else {
            Err(result)
        }
    }

    /// Moves the mouse cursor without generating events.
    #[inline]
    pub fn warp_mouse_cursor_position(point: CGPoint) -> Result<(), CGError> {
        let result = unsafe { CGWarpMouseCursorPosition(point) };
        if result == 0 {
            Ok(())
        } else {
            Err(result)
        }
    }

    /// Connects or disconnects the mouse and cursor while an application is
    /// in the foreground.
    #[inline]
    pub fn associate_mouse_and_mouse_cursor_position(connected: bool) -> Result<(), CGError> {
        let result = unsafe { CGAssociateMouseAndMouseCursorPosition(connected as boolean_t) };
        if result == 0 {
            Ok(())
        } else {
            Err(result)
        }
    }
}

impl CGDisplayMode {
    #[inline]
    pub fn height(&self) -> u64 {
        unsafe { CGDisplayModeGetHeight(self.as_ptr()) as u64 }
    }

    #[inline]
    pub fn width(&self) -> u64 {
        unsafe { CGDisplayModeGetWidth(self.as_ptr()) as u64 }
    }

    #[inline]
    pub fn pixel_height(&self) -> u64 {
        unsafe { CGDisplayModeGetPixelHeight(self.as_ptr()) as u64 }
    }

    #[inline]
    pub fn pixel_width(&self) -> u64 {
        unsafe { CGDisplayModeGetPixelWidth(self.as_ptr()) as u64 }
    }

    #[inline]
    pub fn refresh_rate(&self) -> f64 {
        unsafe { CGDisplayModeGetRefreshRate(self.as_ptr()) }
    }
}

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    pub static CGRectNull: CGRect;
    pub static CGRectInfinite: CGRect;

    fn CGDisplayModeRelease(mode: ::sys::CGDisplayModeRef);

    fn CGMainDisplayID() -> CGDirectDisplayID;
    fn CGDisplayIsActive(display: CGDirectDisplayID) -> boolean_t;
    fn CGDisplayIsAlwaysInMirrorSet(display: CGDirectDisplayID) -> boolean_t;
    fn CGDisplayIsAsleep(display: CGDirectDisplayID) -> boolean_t;
    fn CGDisplayIsBuiltin(display: CGDirectDisplayID) -> boolean_t;
    fn CGDisplayIsInHWMirrorSet(display: CGDirectDisplayID) -> boolean_t;
    fn CGDisplayIsInMirrorSet(display: CGDirectDisplayID) -> boolean_t;
    fn CGDisplayIsMain(display: CGDirectDisplayID) -> boolean_t;
    fn CGDisplayIsOnline(display: CGDirectDisplayID) -> boolean_t;
    fn CGDisplayIsStereo(display: CGDirectDisplayID) -> boolean_t;
    fn CGDisplayMirrorsDisplay(display: CGDirectDisplayID) -> CGDirectDisplayID;
    fn CGDisplayPrimaryDisplay(display: CGDirectDisplayID) -> CGDirectDisplayID;
    fn CGDisplayRotation(display: CGDirectDisplayID) -> libc::c_double;
    fn CGDisplayScreenSize(display: CGDirectDisplayID) -> CGSize;
    fn CGDisplaySerialNumber(display: CGDirectDisplayID) -> libc::uint32_t;
    fn CGDisplayUnitNumber(display: CGDirectDisplayID) -> libc::uint32_t;
    fn CGDisplayUsesOpenGLAcceleration(display: CGDirectDisplayID) -> boolean_t;
    fn CGDisplayVendorNumber(display: CGDirectDisplayID) -> libc::uint32_t;
    fn CGGetActiveDisplayList(
        max_displays: libc::uint32_t,
        active_displays: *mut CGDirectDisplayID,
        display_count: *mut libc::uint32_t,
    ) -> CGError;
    fn CGDisplayModelNumber(display: CGDirectDisplayID) -> libc::uint32_t;
    fn CGDisplayPixelsHigh(display: CGDirectDisplayID) -> libc::size_t;
    fn CGDisplayPixelsWide(display: CGDirectDisplayID) -> libc::size_t;
    fn CGDisplayBounds(display: CGDirectDisplayID) -> CGRect;
    fn CGDisplayCreateImage(display: CGDirectDisplayID) -> ::sys::CGImageRef;

    fn CGDisplayCopyDisplayMode(display: CGDirectDisplayID) -> ::sys::CGDisplayModeRef;
    fn CGDisplayModeGetHeight(mode: ::sys::CGDisplayModeRef) -> libc::size_t;
    fn CGDisplayModeGetWidth(mode: ::sys::CGDisplayModeRef) -> libc::size_t;
    fn CGDisplayModeGetPixelHeight(mode: ::sys::CGDisplayModeRef) -> libc::size_t;
    fn CGDisplayModeGetPixelWidth(mode: ::sys::CGDisplayModeRef) -> libc::size_t;
    fn CGDisplayModeGetRefreshRate(mode: ::sys::CGDisplayModeRef) -> libc::c_double;

    // mouse stuff
    fn CGDisplayHideCursor(display: CGDirectDisplayID) -> CGError;
    fn CGDisplayShowCursor(display: CGDirectDisplayID) -> CGError;
    fn CGDisplayMoveCursorToPoint(display: CGDirectDisplayID, point: CGPoint) -> CGError;
    fn CGWarpMouseCursorPosition(point: CGPoint) -> CGError;
    fn CGAssociateMouseAndMouseCursorPosition(connected: boolean_t) -> CGError;

    // Window Services Reference
    fn CGWindowListCopyWindowInfo(
        option: CGWindowListOption,
        relativeToWindow: CGWindowID,
    ) -> CFArrayRef;
    fn CGWindowListCreateImage(
        screenBounds: CGRect,
        listOptions: CGWindowListOption,
        windowId: CGWindowID,
        imageOptions: CGWindowImageOption,
    ) -> ::sys::CGImageRef;
    fn CGWindowListCreateImageFromArray(
        screenBounds: CGRect,
        windowArray: CFArrayRef,
        imageOptions: CGWindowImageOption,
    ) -> ::sys::CGImageRef;
}
