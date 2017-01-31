// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_upper_case_globals)]

use objc::runtime::Class;
use base::{id, class, BOOL, SEL};
use foundation::{NSInteger, NSUInteger, NSTimeInterval,
                 NSPoint, NSSize, NSRect, NSRectEdge};
use libc;

pub use core_graphics::base::CGFloat;
pub use core_graphics::geometry::CGPoint;

pub use self::NSApplicationActivationPolicy::*;
pub use self::NSApplicationActivationOptions::*;
pub use self::NSWindowMask::*;
pub use self::NSBackingStoreType::*;
pub use self::NSOpenGLPixelFormatAttribute::*;
pub use self::NSOpenGLPFAOpenGLProfiles::*;
pub use self::NSEventType::*;

pub type CGLContextObj = *mut libc::c_void;

pub type GLint = libc::int32_t;

#[link(name = "AppKit", kind = "framework")]
extern {
    pub static NSAppKitVersionNumber: f64;
}

pub const NSAppKitVersionNumber10_0: f64 = 577.0;
pub const NSAppKitVersionNumber10_1: f64 = 620.0;
pub const NSAppKitVersionNumber10_2: f64 = 663.0;
pub const NSAppKitVersionNumber10_2_3: f64 = 663.6;
pub const NSAppKitVersionNumber10_3: f64 = 743.0;
pub const NSAppKitVersionNumber10_3_2: f64 = 743.14;
pub const NSAppKitVersionNumber10_3_3: f64 = 743.2;
pub const NSAppKitVersionNumber10_3_5: f64 = 743.24;
pub const NSAppKitVersionNumber10_3_7: f64 = 743.33;
pub const NSAppKitVersionNumber10_3_9: f64 = 743.36;
pub const NSAppKitVersionNumber10_4: f64 = 824.0;
pub const NSAppKitVersionNumber10_4_1: f64 = 824.1;
pub const NSAppKitVersionNumber10_4_3: f64 = 824.23;
pub const NSAppKitVersionNumber10_4_4: f64 = 824.33;
pub const NSAppKitVersionNumber10_4_7: f64 = 824.41;
pub const NSAppKitVersionNumber10_5: f64 = 949.0;
pub const NSAppKitVersionNumber10_5_2: f64 = 949.27;
pub const NSAppKitVersionNumber10_5_3: f64 = 949.33;
pub const NSAppKitVersionNumber10_6: f64 = 1038.0;
pub const NSAppKitVersionNumber10_7: f64 = 1138.0;
pub const NSAppKitVersionNumber10_7_2: f64 = 1138.23;
pub const NSAppKitVersionNumber10_7_3: f64 = 1138.32;
pub const NSAppKitVersionNumber10_7_4: f64 = 1138.47;
pub const NSAppKitVersionNumber10_8: f64 = 1187.0;
pub const NSAppKitVersionNumber10_9: f64 = 1265.0;

pub unsafe fn NSApp() -> id {
    msg_send![class("NSApplication"), sharedApplication]
}

#[repr(i64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSApplicationActivationPolicy {
    NSApplicationActivationPolicyRegular = 0,
    NSApplicationActivationPolicyAccessory = 1,
    NSApplicationActivationPolicyProhibited = 2,
    NSApplicationActivationPolicyERROR = -1
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSApplicationActivationOptions {
    NSApplicationActivateAllWindows = 1 << 0,
    NSApplicationActivateIgnoringOtherApps = 1 << 1
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSApplicationTerminateReply {
    NSTerminateCancel = 0,
    NSTerminateNow = 1,
    NSTerminateLater = 2,
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSWindowMask {
    NSBorderlessWindowMask      = 0,
    NSTitledWindowMask          = 1 << 0,
    NSClosableWindowMask        = 1 << 1,
    NSMiniaturizableWindowMask  = 1 << 2,
    NSResizableWindowMask       = 1 << 3,

    NSTexturedBackgroundWindowMask  = 1 << 8,

    NSUnifiedTitleAndToolbarWindowMask  = 1 << 12,

    NSFullScreenWindowMask      = 1 << 14,

    NSFullSizeContentViewWindowMask = 1 << 15
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSWindowTitleVisibility {
    NSWindowTitleVisible = 0,
    NSWindowTitleHidden = 1
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSBackingStoreType {
    NSBackingStoreRetained      = 0,
    NSBackingStoreNonretained   = 1,
    NSBackingStoreBuffered      = 2
}

bitflags! {
    flags NSWindowOrderingMode: NSInteger {
        const NSWindowAbove =  1,
        const NSWindowBelow = -1,
        const NSWindowOut   =  0,
    }
}

bitflags! {
    flags NSAlignmentOptions: libc::c_ulonglong {
        const NSAlignMinXInward         = 1 << 0,
        const NSAlignMinYInward         = 1 << 1,
        const NSAlignMaxXInward         = 1 << 2,
        const NSAlignMaxYInward         = 1 << 3,
        const NSAlignWidthInward        = 1 << 4,
        const NSAlignHeightInward       = 1 << 5,
        const NSAlignMinXOutward        = 1 << 8,
        const NSAlignMinYOutward        = 1 << 9,
        const NSAlignMaxXOutward        = 1 << 10,
        const NSAlignMaxYOutward        = 1 << 11,
        const NSAlignWidthOutward       = 1 << 12,
        const NSAlignHeightOutward      = 1 << 13,
        const NSAlignMinXNearest        = 1 << 16,
        const NSAlignMinYNearest        = 1 << 17,
        const NSAlignMaxXNearest        = 1 << 18,
        const NSAlignMaxYNearest        = 1 << 19,
        const NSAlignWidthNearest       = 1 << 20,
        const NSAlignHeightNearest      = 1 << 21,
        const NSAlignRectFlipped        = 1 << 63,
        const NSAlignAllEdgesInward     = NSAlignMinXInward.bits
                                        | NSAlignMaxXInward.bits
                                        | NSAlignMinYInward.bits
                                        | NSAlignMaxYInward.bits,
        const NSAlignAllEdgesOutward    = NSAlignMinXOutward.bits
                                        | NSAlignMaxXOutward.bits
                                        | NSAlignMinYOutward.bits
                                        | NSAlignMaxYOutward.bits,
        const NSAlignAllEdgesNearest    = NSAlignMinXNearest.bits
                                        | NSAlignMaxXNearest.bits
                                        | NSAlignMinYNearest.bits
                                        | NSAlignMaxYNearest.bits,
    }
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSOpenGLPixelFormatAttribute {
    NSOpenGLPFAAllRenderers             = 1,
    NSOpenGLPFATripleBuffer             = 3,
    NSOpenGLPFADoubleBuffer             = 5,
    NSOpenGLPFAStereo                   = 6,
    NSOpenGLPFAAuxBuffers               = 7,
    NSOpenGLPFAColorSize                = 8,
    NSOpenGLPFAAlphaSize                = 11,
    NSOpenGLPFADepthSize                = 12,
    NSOpenGLPFAStencilSize              = 13,
    NSOpenGLPFAAccumSize                = 14,
    NSOpenGLPFAMinimumPolicy            = 51,
    NSOpenGLPFAMaximumPolicy            = 52,
    NSOpenGLPFAOffScreen                = 53,
    NSOpenGLPFAFullScreen               = 54,
    NSOpenGLPFASampleBuffers            = 55,
    NSOpenGLPFASamples                  = 56,
    NSOpenGLPFAAuxDepthStencil          = 57,
    NSOpenGLPFAColorFloat               = 58,
    NSOpenGLPFAMultisample              = 59,
    NSOpenGLPFASupersample              = 60,
    NSOpenGLPFASampleAlpha              = 61,
    NSOpenGLPFARendererID               = 70,
    NSOpenGLPFASingleRenderer           = 71,
    NSOpenGLPFANoRecovery               = 72,
    NSOpenGLPFAAccelerated              = 73,
    NSOpenGLPFAClosestPolicy            = 74,
    NSOpenGLPFARobust                   = 75,
    NSOpenGLPFABackingStore             = 76,
    NSOpenGLPFAMPSafe                   = 78,
    NSOpenGLPFAWindow                   = 80,
    NSOpenGLPFAMultiScreen              = 81,
    NSOpenGLPFACompliant                = 83,
    NSOpenGLPFAScreenMask               = 84,
    NSOpenGLPFAPixelBuffer              = 90,
    NSOpenGLPFARemotePixelBuffer        = 91,
    NSOpenGLPFAAllowOfflineRenderers    = 96,
    NSOpenGLPFAAcceleratedCompute       = 97,
    NSOpenGLPFAOpenGLProfile            = 99,
    NSOpenGLPFAVirtualScreenCount       = 128,
}

#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSOpenGLPFAOpenGLProfiles {
    NSOpenGLProfileVersionLegacy = 0x1000,
    NSOpenGLProfileVersion3_2Core = 0x3200,
    NSOpenGLProfileVersion4_1Core = 0x4100,
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSOpenGLContextParameter {
    NSOpenGLCPSwapInterval          = 222,
    NSOpenGLCPSurfaceOrder          = 235,
    NSOpenGLCPSurfaceOpacity        = 236,
    NSOpenGLCPSurfaceBackingSize    = 304,
    NSOpenGLCPReclaimResources      = 308,
    NSOpenGLCPCurrentRendererID     = 309,
    NSOpenGLCPGPUVertexProcessing   = 310,
    NSOpenGLCPGPUFragmentProcessing = 311,
    NSOpenGLCPHasDrawable           = 314,
    NSOpenGLCPMPSwapsInFlight       = 315,
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSWindowButton {
    NSWindowCloseButton            = 0,
    NSWindowMiniaturizeButton      = 1,
    NSWindowZoomButton             = 2,
    NSWindowToolbarButton          = 3,
    NSWindowDocumentIconButton     = 4,
    NSWindowDocumentVersionsButton = 6,
    NSWindowFullScreenButton       = 7,
}

pub static NSMainMenuWindowLevel: libc::int32_t = 24;

pub trait NSApplication {
    unsafe fn sharedApplication(_: Self) -> id {
        msg_send![class("NSApplication"), sharedApplication]
    }

    unsafe fn setActivationPolicy_(self, policy: NSApplicationActivationPolicy) -> BOOL;
    unsafe fn setMainMenu_(self, menu: id);
    unsafe fn setServicesMenu_(self, menu: id);
    unsafe fn activateIgnoringOtherApps_(self, ignore: BOOL);
    unsafe fn run(self);
    unsafe fn finishLaunching(self);
    unsafe fn nextEventMatchingMask_untilDate_inMode_dequeue_(self,
                                                              mask: NSUInteger,
                                                              expiration: id,
                                                              in_mode: id,
                                                              dequeue: BOOL) -> id;
    unsafe fn sendEvent_(self, an_event: id);
    unsafe fn postEvent_atStart_(self, anEvent: id, flag: BOOL);
    unsafe fn stop_(self, sender: id);
}

impl NSApplication for id {
    unsafe fn setActivationPolicy_(self, policy: NSApplicationActivationPolicy) -> BOOL {
        msg_send![self, setActivationPolicy:policy as NSInteger]
    }

    unsafe fn setMainMenu_(self, menu: id) {
        msg_send![self, setMainMenu:menu]
    }

    unsafe fn setServicesMenu_(self, menu: id) {
        msg_send![self, setServicesMenu:menu]
    }

    unsafe fn activateIgnoringOtherApps_(self, ignore: BOOL) {
        msg_send![self, activateIgnoringOtherApps:ignore]
    }

    unsafe fn run(self) {
        msg_send![self, run]
    }

    unsafe fn finishLaunching(self) {
        msg_send![self, finishLaunching]
    }

    unsafe fn nextEventMatchingMask_untilDate_inMode_dequeue_(self,
                                                              mask: NSUInteger,
                                                              expiration: id,
                                                              in_mode: id,
                                                              dequeue: BOOL) -> id {
        msg_send![self, nextEventMatchingMask:mask
                                    untilDate:expiration
                                       inMode:in_mode
                                      dequeue:dequeue]
    }

    unsafe fn sendEvent_(self, an_event: id) {
        msg_send![self, sendEvent:an_event]
    }

    unsafe fn postEvent_atStart_(self, anEvent: id, flag: BOOL) {
        msg_send![self, postEvent:anEvent atStart:flag]
    }

    unsafe fn stop_(self, sender: id) {
        msg_send![self, stop:sender]
    }
}

pub trait NSRunningApplication {
    unsafe fn currentApplication(_: Self) -> id {
        msg_send![class("NSRunningApplication"), currentApplication]
    }
    unsafe fn activateWithOptions_(self, options: NSApplicationActivationOptions) -> BOOL;
}

impl NSRunningApplication for id {
    unsafe fn activateWithOptions_(self, options: NSApplicationActivationOptions) -> BOOL {
        msg_send![self, activateWithOptions:options as NSUInteger]
    }
}

pub trait NSMenu {
    unsafe fn new(_: Self) -> id {
        msg_send![class("NSMenu"), new]
    }

    unsafe fn setAutoenablesItems(self, state: BOOL);

    unsafe fn addItem_(self, menu_item: id);
    unsafe fn addItemWithTitle_action_keyEquivalent(self, title: id, action: SEL, key: id) -> id;
}

impl NSMenu for id {
    unsafe fn setAutoenablesItems(self, state: BOOL) {
        msg_send![self, setAutoenablesItems: state]
    }

    unsafe fn addItem_(self, menu_item: id) {
        msg_send![self, addItem:menu_item]
    }

    unsafe fn addItemWithTitle_action_keyEquivalent(self, title: id, action: SEL, key: id) -> id {
        msg_send![self, addItemWithTitle:title action:action keyEquivalent:key]
    }
}

pub trait NSMenuItem {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class("NSMenuItem"), alloc]
    }

    unsafe fn new(_: Self) -> id {
        msg_send![class("NSMenuItem"), new]
    }

    unsafe fn separatorItem(_: Self) -> id {
        msg_send![class("NSMenuItem"), separatorItem]
    }

    unsafe fn initWithTitle_action_keyEquivalent_(self, title: id, action: SEL, key: id) -> id;
    unsafe fn setKeyEquivalentModifierMask_(self, mask: NSEventModifierFlags);
    unsafe fn setSubmenu_(self, submenu: id);
}

impl NSMenuItem for id {
    unsafe fn initWithTitle_action_keyEquivalent_(self, title: id, action: SEL, key: id) -> id {
        msg_send![self, initWithTitle:title action:action keyEquivalent:key]
    }

    unsafe fn setKeyEquivalentModifierMask_(self, mask: NSEventModifierFlags) {
        msg_send![self, setKeyEquivalentModifierMask:mask]
    }

    unsafe fn setSubmenu_(self, submenu: id) {
        msg_send![self, setSubmenu:submenu]
    }
}

pub type NSWindowDepth = libc::c_int;

bitflags! {
    flags NSWindowCollectionBehavior: NSUInteger {
        const NSWindowCollectionBehaviorDefault = 0,
        const NSWindowCollectionBehaviorCanJoinAllSpaces = 1 << 0,
        const NSWindowCollectionBehaviorMoveToActiveSpace = 1 << 1,

        const NSWindowCollectionBehaviorManaged = 1 << 2,
        const NSWindowCollectionBehaviorTransient = 1 << 3,
        const NSWindowCollectionBehaviorStationary = 1 << 4,

        const NSWindowCollectionBehaviorParticipatesInCycle = 1 << 5,
        const NSWindowCollectionBehaviorIgnoresCycle = 1 << 6,

        const NSWindowCollectionBehaviorFullScreenPrimary = 1 << 7,
        const NSWindowCollectionBehaviorFullScreenAuxiliary = 1 << 8,
    }
}

bitflags! {
    flags NSWindowOcclusionState: NSUInteger {
        const NSWindowOcclusionStateVisible = 1 << 1
    }
}

#[link(name = "wbwindow")]
extern {
    #[link_name = "OBJC_CLASS_$_WBWindow"]
    static WBWINDOW_C: Class;
}

pub trait NSWindow {
    unsafe fn alloc(_: Self) -> id {
        msg_send![&WBWINDOW_C, alloc]
    }

    // Creating Windows
    unsafe fn initWithContentRect_styleMask_backing_defer_(self,
                                                           rect: NSRect,
                                                           style: NSUInteger,
                                                           backing: NSBackingStoreType,
                                                           defer: BOOL) -> id;
    unsafe fn initWithContentRect_styleMask_backing_defer_screen_(self,
                                                                  rect: NSRect,
                                                                  style: NSUInteger,
                                                                  backing: NSBackingStoreType,
                                                                  defer: BOOL,
                                                                  screen: id) -> id;

    // Configuring Windows
    unsafe fn styleMask(self) -> NSUInteger;
    unsafe fn setStyleMask_(self, styleMask: NSUInteger);
    unsafe fn toggleFullScreen_(self, sender: id);
    unsafe fn worksWhenModal(self) -> BOOL;
    unsafe fn alphaValue(self) -> CGFloat;
    unsafe fn setAlphaValue_(self, windowAlpha: CGFloat);
    unsafe fn backgroundColor(self) -> id;
    unsafe fn setBackgroundColor_(self, color: id);
    unsafe fn colorSpace(self) -> id;
    unsafe fn setColorSpace_(self, colorSpace: id);
    unsafe fn contentView(self) -> id;
    unsafe fn setContentView_(self, view: id);
    unsafe fn canHide(self) -> BOOL;
    unsafe fn setCanHide_(self, canHide: BOOL);
    unsafe fn hidesOnDeactivate(self) -> BOOL;
    unsafe fn setHidesOnDeactivate_(self, hideOnDeactivate: BOOL);
    unsafe fn collectionBehavior(self) -> NSWindowCollectionBehavior;
    unsafe fn setCollectionBehavior_(self, collectionBehavior: NSWindowCollectionBehavior);
    unsafe fn setOpaque_(self, opaque: BOOL);
    unsafe fn hasShadow(self) -> BOOL;
    unsafe fn setHasShadow_(self, hasShadow: BOOL);
    unsafe fn invalidateShadow(self);
    unsafe fn autorecalculatesContentBorderThicknessForEdge_(self, edge: NSRectEdge) -> BOOL;
    unsafe fn setAutorecalculatesContentBorderThickness_forEdge_(self,
                                                                 autorecalculateContentBorderThickness: BOOL,
                                                                 edge: NSRectEdge) -> BOOL;
    unsafe fn contentBorderThicknessForEdge_(self, edge: NSRectEdge) -> CGFloat;
    unsafe fn setContentBorderThickness_forEdge_(self, borderThickness: CGFloat, edge: NSRectEdge);
    unsafe fn delegate(self) -> id;
    unsafe fn setDelegate_(self, delegate: id);
    unsafe fn preventsApplicationTerminationWhenModal(self) -> BOOL;
    unsafe fn setPreventsApplicationTerminationWhenModal_(self, flag: BOOL);

    // TODO: Accessing Window Information

    // Getting Layout Information
    unsafe fn contentRectForFrameRect_styleMask_(self, windowFrame: NSRect, windowStyle: NSUInteger) -> NSRect;
    unsafe fn frameRectForContentRect_styleMask_(self, windowContentRect: NSRect, windowStyle: NSUInteger) -> NSRect;
    unsafe fn minFrameWidthWithTitle_styleMask_(self, windowTitle: id, windowStyle: NSUInteger) -> CGFloat;
    unsafe fn contentRectForFrameRect_(self, windowFrame: NSRect) -> NSRect;
    unsafe fn frameRectForContentRect_(self, windowContent: NSRect) -> NSRect;

    // Managing Windows
    unsafe fn drawers(self) -> id;
    unsafe fn windowController(self) -> id;
    unsafe fn setWindowController_(self, windowController: id);

    // TODO: Managing Sheets

    // Sizing Windows
    unsafe fn frame(self) -> NSRect;
    unsafe fn setFrameOrigin_(self, point: NSPoint);
    unsafe fn setFrameTopLeftPoint_(self, point: NSPoint);
    unsafe fn constrainFrameRect_toScreen_(self, frameRect: NSRect, screen: id);
    unsafe fn cascadeTopLeftFromPoint_(self, topLeft: NSPoint) -> NSPoint;
    unsafe fn setFrame_display_(self, windowFrame: NSRect, display: BOOL);
    unsafe fn setFrame_displayViews_(self, windowFrame: NSRect, display: BOOL);
    unsafe fn aspectRatio(self) -> NSSize;
    unsafe fn setAspectRatio_(self, aspectRatio: NSSize);
    unsafe fn minSize(self) -> NSSize;
    unsafe fn setMinSize_(self, minSize: NSSize);
    unsafe fn maxSize(self) -> NSSize;
    unsafe fn setMaxSize_(self, maxSize: NSSize);
    unsafe fn performZoom_(self, sender: id);
    unsafe fn zoom_(self, sender: id);
    unsafe fn resizeFlags(self) -> NSInteger;
    unsafe fn showsResizeIndicator(self) -> BOOL;
    unsafe fn setShowsResizeIndicator_(self, showsResizeIndicator: BOOL);
    unsafe fn resizeIncrements(self) -> NSSize;
    unsafe fn setResizeIncrements_(self, resizeIncrements: NSSize);
    unsafe fn preservesContentDuringLiveResize(self) -> BOOL;
    unsafe fn setPreservesContentDuringLiveResize_(self, preservesContentDuringLiveResize: BOOL);
    unsafe fn inLiveResize(self) -> BOOL;

    // Sizing Content
    unsafe fn contentAspectRatio(self) -> NSSize;
    unsafe fn setContentAspectRatio_(self, contentAspectRatio: NSSize);
    unsafe fn contentMinSize(self) -> NSSize;
    unsafe fn setContentMinSize_(self, contentMinSize: NSSize);
    unsafe fn contentSize(self) -> NSSize;
    unsafe fn setContentSize_(self, contentSize: NSSize);
    unsafe fn contentMaxSize(self) -> NSSize;
    unsafe fn setContentMaxSize_(self, contentMaxSize: NSSize);
    unsafe fn contentResizeIncrements(self) -> NSSize;
    unsafe fn setContentResizeIncrements_(self, contentResizeIncrements: NSSize);

    // Managing Window Visibility and Occlusion State
    unsafe fn isVisible(self) -> BOOL; // NOTE: Deprecated in 10.9
    unsafe fn occlusionState(self) -> NSWindowOcclusionState;

    // Managing Window Layers
    unsafe fn orderOut_(self, sender: id);
    unsafe fn orderBack_(self, sender: id);
    unsafe fn orderFront_(self, sender: id);
    unsafe fn orderFrontRegardless(self);
    unsafe fn orderFrontWindow_relativeTo_(self, orderingMode: NSWindowOrderingMode, otherWindowNumber: NSInteger);
    unsafe fn level(self) -> NSInteger;
    unsafe fn setLevel_(self, level: NSInteger);

    // Managing Key Status
    unsafe fn canBecomeKeyWindow(self) -> BOOL;
    unsafe fn makeKeyWindow(self);
    unsafe fn makeKeyAndOrderFront_(self, sender: id);
    // skipped: becomeKeyWindow (should not be invoked directly, according to Apple's documentation)
    // skipped: resignKeyWindow (should not be invoked directly, according to Apple's documentation)

    // Managing Main Status
    unsafe fn canBecomeMainWindow(self) -> BOOL;
    unsafe fn makeMainWindow(self);
    // skipped: becomeMainWindow (should not be invoked directly, according to Apple's documentation)
    // skipped: resignMainWindow (should not be invoked directly, according to Apple's documentation)

    // TODO: Managing Toolbars
    // TODO: Managing Attached Windows
    // TODO: Managing Window Buffers
    // TODO: Managing Default Buttons
    // TODO: Managing Field Editors
    // TODO: Managing the Window Menu
    // TODO: Managing Cursor Rectangles

    // Managing Title Bars
    unsafe fn standardWindowButton_(self, windowButtonKind: NSWindowButton) -> id;

    // TODO: Managing Tooltips
    // TODO: Handling Events

    // Managing Responders
    unsafe fn initialFirstResponder(self) -> id;
    unsafe fn firstResponder(self) -> id;
    unsafe fn setInitialFirstResponder_(self, responder: id);
    unsafe fn makeFirstResponder_(self, responder: id) -> BOOL;

    // TODO: Managing the Key View Loop

    // Handling Keyboard Events
    unsafe fn keyDown_(self, event: id);

    // Handling Mouse Events
    unsafe fn acceptsMouseMovedEvents(self) -> BOOL;
    unsafe fn ignoresMouseEvents(self) -> BOOL;
    unsafe fn setIgnoresMouseEvents_(self, ignoreMouseEvents: BOOL);
    unsafe fn mouseLocationOutsideOfEventStream(self) -> NSPoint;
    unsafe fn setAcceptsMouseMovedEvents_(self, acceptMouseMovedEvents: BOOL);
    unsafe fn windowNumberAtPoint_belowWindowWithWindowNumber_(self,
                                                               point: NSPoint,
                                                               windowNumber: NSInteger) -> NSInteger;

    // TODO: Handling Window Restoration
    // TODO: Bracketing Drawing Operations
    // TODO: Drawing Windows
    // TODO: Window Animation
    // TODO: Updating Windows
    // TODO: Dragging Items

    // Converting Coordinates
    unsafe fn backingScaleFactor(self) -> CGFloat;
    unsafe fn backingAlignedRect_options_(self, rect: NSRect, options: NSAlignmentOptions) -> NSRect;
    unsafe fn convertRectFromBacking_(self, rect: NSRect) -> NSRect;
    unsafe fn convertRectToBacking_(self, rect: NSRect) -> NSRect;
    unsafe fn convertRectToScreen_(self, rect: NSRect) -> NSRect;
    unsafe fn convertRectFromScreen_(self, rect: NSRect) -> NSRect;

    // Accessing Edited Status
    unsafe fn setDocumentEdited_(self, documentEdited: BOOL);

    // Managing Titles
    unsafe fn title(self) -> id;
    unsafe fn setTitle_(self, title: id);
    unsafe fn setTitleWithRepresentedFilename_(self, filePath: id);
    unsafe fn setTitleVisibility_(self, visibility: NSWindowTitleVisibility);
    unsafe fn setTitlebarAppearsTransparent_(self, transparent: BOOL);
    unsafe fn representedFilename(self) -> id;
    unsafe fn setRepresentedFilename_(self, filePath: id);
    unsafe fn representedURL(self) -> id;
    unsafe fn setRepresentedURL_(self, representedURL: id);

    // Accessing Screen Information
    unsafe fn screen(self) -> id;
    unsafe fn deepestScreen(self) -> id;
    unsafe fn displaysWhenScreenProfileChanges(self) -> BOOL;
    unsafe fn setDisplaysWhenScreenProfileChanges_(self, displaysWhenScreenProfileChanges: BOOL);

    // Moving Windows
    unsafe fn setMovableByWindowBackground_(self, movableByWindowBackground: BOOL);
    unsafe fn setMovable_(self, movable: BOOL);
    unsafe fn center(self);

    // Closing Windows
    unsafe fn performClose_(self, sender: id);
    unsafe fn close(self);
    unsafe fn setReleasedWhenClosed_(self, releasedWhenClosed: BOOL);

    // Minimizing Windows
    unsafe fn performMiniaturize_(self, sender: id);
    unsafe fn miniaturize_(self, sender: id);
    unsafe fn deminiaturize_(self, sender: id);
    unsafe fn miniwindowImage(self) -> id;
    unsafe fn setMiniwindowImage_(self, miniwindowImage: id);
    unsafe fn miniwindowTitle(self) -> id;
    unsafe fn setMiniwindowTitle_(self, miniwindowTitle: id);

    // TODO: Getting the Dock Tile
    // TODO: Printing Windows
    // TODO: Providing Services
    // TODO: Working with Carbon
    // TODO: Triggering Constraint-Based Layout
    // TODO: Debugging Constraint-Based Layout
    // TODO: Constraint-Based Layouts
}

impl NSWindow for id {
    // Creating Windows

    unsafe fn initWithContentRect_styleMask_backing_defer_(self,
                                                           rect: NSRect,
                                                           style: NSUInteger,
                                                           backing: NSBackingStoreType,
                                                           defer: BOOL) -> id {
        msg_send![self, initWithContentRect:rect
                                  styleMask:style
                                    backing:backing as NSUInteger
                                      defer:defer]
    }

    unsafe fn initWithContentRect_styleMask_backing_defer_screen_(self,
                                                                  rect: NSRect,
                                                                  style: NSUInteger,
                                                                  backing: NSBackingStoreType,
                                                                  defer: BOOL,
                                                                  screen: id) -> id {
        msg_send![self, initWithContentRect:rect
                                  styleMask:style
                                    backing:backing as NSUInteger
                                      defer:defer
                                     screen:screen]
    }

    // Configuring Windows

    unsafe fn styleMask(self) -> NSUInteger {
        msg_send![self, styleMask]
    }

    unsafe fn setStyleMask_(self, styleMask: NSUInteger) {
        msg_send![self, setStyleMask:styleMask]
    }

    unsafe fn toggleFullScreen_(self, sender: id) {
        msg_send![self, toggleFullScreen:sender]
    }

    unsafe fn worksWhenModal(self) -> BOOL {
        msg_send![self, worksWhenModal]
    }

    unsafe fn alphaValue(self) -> CGFloat {
        msg_send![self, alphaValue]
    }

    unsafe fn setAlphaValue_(self, windowAlpha: CGFloat) {
        msg_send![self, setAlphaValue:windowAlpha]
    }

    unsafe fn backgroundColor(self) -> id {
        msg_send![self, backgroundColor]
    }

    unsafe fn setBackgroundColor_(self, color: id) {
        msg_send![self, setBackgroundColor:color]
    }

    unsafe fn colorSpace(self) -> id {
        msg_send![self, colorSpace]
    }

    unsafe fn setColorSpace_(self, colorSpace: id) {
        msg_send![self, setColorSpace:colorSpace]
    }

    unsafe fn contentView(self) -> id {
        msg_send![self, contentView]
    }

    unsafe fn setContentView_(self, view: id) {
        msg_send![self, setContentView:view]
    }

    unsafe fn canHide(self) -> BOOL {
        msg_send![self, canHide]
    }

    unsafe fn setCanHide_(self, canHide: BOOL) {
        msg_send![self, setCanHide:canHide]
    }

    unsafe fn hidesOnDeactivate(self) -> BOOL {
        msg_send![self, hidesOnDeactivate]
    }

    unsafe fn setHidesOnDeactivate_(self, hideOnDeactivate: BOOL) {
        msg_send![self, setHidesOnDeactivate:hideOnDeactivate]
    }

    unsafe fn collectionBehavior(self) -> NSWindowCollectionBehavior {
        msg_send![self, collectionBehavior]
    }

    unsafe fn setCollectionBehavior_(self, collectionBehavior: NSWindowCollectionBehavior) {
        msg_send![self, setCollectionBehavior:collectionBehavior]
    }

    unsafe fn setOpaque_(self, opaque: BOOL) {
        msg_send![self, setOpaque:opaque]
    }

    unsafe fn hasShadow(self) -> BOOL {
        msg_send![self, hasShadow]
    }

    unsafe fn setHasShadow_(self, hasShadow: BOOL) {
        msg_send![self, setHasShadow:hasShadow]
    }

    unsafe fn invalidateShadow(self) {
        msg_send![self, invalidateShadow]
    }

    unsafe fn autorecalculatesContentBorderThicknessForEdge_(self, edge: NSRectEdge) -> BOOL {
        msg_send![self, autorecalculatesContentBorderThicknessForEdge:edge]
    }

    unsafe fn setAutorecalculatesContentBorderThickness_forEdge_(self,
                                                                 autorecalculateContentBorderThickness: BOOL,
                                                                 edge: NSRectEdge) -> BOOL {
        msg_send![self, setAutorecalculatesContentBorderThickness:
                        autorecalculateContentBorderThickness forEdge:edge]
    }

    unsafe fn contentBorderThicknessForEdge_(self, edge: NSRectEdge) -> CGFloat {
        msg_send![self, contentBorderThicknessForEdge:edge]
    }

    unsafe fn setContentBorderThickness_forEdge_(self, borderThickness: CGFloat, edge: NSRectEdge) {
        msg_send![self, setContentBorderThickness:borderThickness forEdge:edge]
    }

    unsafe fn delegate(self) -> id {
        msg_send![self, delegate]
    }

    unsafe fn setDelegate_(self, delegate: id) {
        msg_send![self, setDelegate:delegate]
    }

    unsafe fn preventsApplicationTerminationWhenModal(self) -> BOOL {
        msg_send![self, preventsApplicationTerminationWhenModal]
    }

    unsafe fn setPreventsApplicationTerminationWhenModal_(self, flag: BOOL) {
        msg_send![self, setPreventsApplicationTerminationWhenModal:flag]
    }

    // TODO: Accessing Window Information

    // Getting Layout Information

    unsafe fn contentRectForFrameRect_styleMask_(self, windowFrame: NSRect, windowStyle: NSUInteger) -> NSRect {
        msg_send![self, contentRectForFrameRect:windowFrame styleMask:windowStyle]
    }

    unsafe fn frameRectForContentRect_styleMask_(self, windowContentRect: NSRect, windowStyle: NSUInteger) -> NSRect {
        msg_send![self, frameRectForContentRect:windowContentRect styleMask:windowStyle]
    }

    unsafe fn minFrameWidthWithTitle_styleMask_(self, windowTitle: id, windowStyle: NSUInteger) -> CGFloat {
        msg_send![self, minFrameWidthWithTitle:windowTitle styleMask:windowStyle]
    }

    unsafe fn contentRectForFrameRect_(self, windowFrame: NSRect) -> NSRect {
        msg_send![self, contentRectForFrameRect:windowFrame]
    }

    unsafe fn frameRectForContentRect_(self, windowContent: NSRect) -> NSRect {
        msg_send![self, frameRectForContentRect:windowContent]
    }

    // Managing Windows

    unsafe fn drawers(self) -> id {
        msg_send![self, drawers]
    }

    unsafe fn windowController(self) -> id {
        msg_send![self, windowController]
    }

    unsafe fn setWindowController_(self, windowController: id) {
        msg_send![self, setWindowController:windowController]
    }

    // TODO: Managing Sheets

    // Sizing Windows

    unsafe fn frame(self) -> NSRect {
        msg_send![self, frame]
    }

    unsafe fn setFrameOrigin_(self, point: NSPoint) {
        msg_send![self, setFrameOrigin:point]
    }

    unsafe fn setFrameTopLeftPoint_(self, point: NSPoint) {
        msg_send![self, setFrameTopLeftPoint:point]
    }

    unsafe fn constrainFrameRect_toScreen_(self, frameRect: NSRect, screen: id) {
        msg_send![self, constrainFrameRect:frameRect toScreen:screen]
    }

    unsafe fn cascadeTopLeftFromPoint_(self, topLeft: NSPoint) -> NSPoint {
        msg_send![self, cascadeTopLeftFromPoint:topLeft]
    }

    unsafe fn setFrame_display_(self, windowFrame: NSRect, display: BOOL) {
        msg_send![self, setFrame:windowFrame display:display]
    }

    unsafe fn setFrame_displayViews_(self, windowFrame: NSRect, display: BOOL) {
        msg_send![self, setFrame:windowFrame displayViews:display]
    }

    unsafe fn aspectRatio(self) -> NSSize {
        msg_send![self, aspectRatio]
    }

    unsafe fn setAspectRatio_(self, aspectRatio: NSSize) {
        msg_send![self, setAspectRatio:aspectRatio]
    }

    unsafe fn minSize(self) -> NSSize {
        msg_send![self, minSize]
    }

    unsafe fn setMinSize_(self, minSize: NSSize) {
        msg_send![self, setMinSize:minSize]
    }

    unsafe fn maxSize(self) -> NSSize {
        msg_send![self, maxSize]
    }

    unsafe fn setMaxSize_(self, maxSize: NSSize) {
        msg_send![self, setMaxSize:maxSize]
    }

    unsafe fn performZoom_(self, sender: id) {
        msg_send![self, performZoom:sender]
    }

    unsafe fn zoom_(self, sender: id) {
        msg_send![self, zoom:sender]
    }

    unsafe fn resizeFlags(self) -> NSInteger {
        msg_send![self, resizeFlags]
    }

    unsafe fn showsResizeIndicator(self) -> BOOL {
        msg_send![self, showsResizeIndicator]
    }

    unsafe fn setShowsResizeIndicator_(self, showsResizeIndicator: BOOL) {
        msg_send![self, setShowsResizeIndicator:showsResizeIndicator]
    }

    unsafe fn resizeIncrements(self) -> NSSize {
        msg_send![self, resizeIncrements]
    }

    unsafe fn setResizeIncrements_(self, resizeIncrements: NSSize) {
        msg_send![self, setResizeIncrements:resizeIncrements]
    }

    unsafe fn preservesContentDuringLiveResize(self) -> BOOL {
        msg_send![self, preservesContentDuringLiveResize]
    }

    unsafe fn setPreservesContentDuringLiveResize_(self, preservesContentDuringLiveResize: BOOL) {
        msg_send![self, setPreservesContentDuringLiveResize:preservesContentDuringLiveResize]
    }

    unsafe fn inLiveResize(self) -> BOOL {
        msg_send![self, inLiveResize]
    }

    // Sizing Content

    unsafe fn contentAspectRatio(self) -> NSSize {
        msg_send![self, contentAspectRatio]
    }

    unsafe fn setContentAspectRatio_(self, contentAspectRatio: NSSize) {
        msg_send![self, setContentAspectRatio:contentAspectRatio]
    }

    unsafe fn contentMinSize(self) -> NSSize {
        msg_send![self, contentMinSize]
    }

    unsafe fn setContentMinSize_(self, contentMinSize: NSSize) {
        msg_send![self, setContentMinSize:contentMinSize]
    }

    unsafe fn contentSize(self) -> NSSize {
        msg_send![self, contentSize]
    }

    unsafe fn setContentSize_(self, contentSize: NSSize) {
        msg_send![self, setContentSize:contentSize]
    }

    unsafe fn contentMaxSize(self) -> NSSize {
        msg_send![self, contentMaxSize]
    }

    unsafe fn setContentMaxSize_(self, contentMaxSize: NSSize) {
        msg_send![self, setContentMaxSize:contentMaxSize]
    }

    unsafe fn contentResizeIncrements(self) -> NSSize {
        msg_send![self, contentResizeIncrements]
    }

    unsafe fn setContentResizeIncrements_(self, contentResizeIncrements: NSSize) {
        msg_send![self, setContentResizeIncrements:contentResizeIncrements]
    }

    // Managing Window Visibility and Occlusion State

    unsafe fn isVisible(self) -> BOOL {
        msg_send![self, isVisible]
    }

    unsafe fn occlusionState(self) -> NSWindowOcclusionState {
        msg_send![self, occlusionState]
    }

    // Managing Window Layers

    unsafe fn orderOut_(self, sender: id) {
        msg_send![self, orderOut:sender]
    }

    unsafe fn orderBack_(self, sender: id) {
        msg_send![self, orderBack:sender]
    }

    unsafe fn orderFront_(self, sender: id) {
        msg_send![self, orderFront:sender]
    }

    unsafe fn orderFrontRegardless(self) {
        msg_send![self, orderFrontRegardless]
    }

    unsafe fn orderFrontWindow_relativeTo_(self, ordering_mode: NSWindowOrderingMode, other_window_number: NSInteger) {
        msg_send![self, orderWindow:ordering_mode relativeTo:other_window_number]
    }

    unsafe fn level(self) -> NSInteger {
        msg_send![self, level]
    }

    unsafe fn setLevel_(self, level: NSInteger) {
        msg_send![self, setLevel:level]
    }

    // Managing Key Status

    unsafe fn canBecomeKeyWindow(self) -> BOOL {
        msg_send![self, canBecomeKeyWindow]
    }

    unsafe fn makeKeyWindow(self) {
        msg_send![self, makeKeyWindow]
    }

    unsafe fn makeKeyAndOrderFront_(self, sender: id) {
        msg_send![self, makeKeyAndOrderFront:sender]
    }

    // Managing Main Status

    unsafe fn canBecomeMainWindow(self) -> BOOL {
        msg_send![self, canBecomeMainWindow]
    }

    unsafe fn makeMainWindow(self) {
        msg_send![self, makeMainWindow]
    }

    // TODO: Managing Toolbars
    // TODO: Managing Attached Windows
    // TODO: Managing Window Buffers
    // TODO: Managing Default Buttons
    // TODO: Managing Field Editors
    // TODO: Managing the Window Menu
    // TODO: Managing Cursor Rectangles

    // Managing Title Bars

    unsafe fn standardWindowButton_(self, windowButtonKind: NSWindowButton) -> id {
        msg_send![self, standardWindowButton:windowButtonKind]
    }

    // TODO: Managing Tooltips
    // TODO: Handling Events

    // Managing Responders

    unsafe fn initialFirstResponder(self) -> id {
        msg_send![self, initialFirstResponder]
    }

    unsafe fn firstResponder(self) -> id {
        msg_send![self, firstResponder]
    }

    unsafe fn setInitialFirstResponder_(self, responder: id) {
        msg_send![self, setInitialFirstResponder:responder]
    }

    unsafe fn makeFirstResponder_(self, responder: id) -> BOOL {
        msg_send![self, makeFirstResponder:responder]
    }

    // TODO: Managing the Key View Loop

    // Handling Keyboard Events

    unsafe fn keyDown_(self, event: id) {
        msg_send![self, keyDown:event]
    }

    // Handling Mouse Events

    unsafe fn acceptsMouseMovedEvents(self) -> BOOL {
        msg_send![self, acceptsMouseMovedEvents]
    }

    unsafe fn ignoresMouseEvents(self) -> BOOL {
        msg_send![self, ignoresMouseEvents]
    }

    unsafe fn setIgnoresMouseEvents_(self, ignoreMouseEvents: BOOL) {
        msg_send![self, setIgnoresMouseEvents:ignoreMouseEvents]
    }

    unsafe fn mouseLocationOutsideOfEventStream(self) -> NSPoint {
        msg_send![self, mouseLocationOutsideOfEventStream]
    }

    unsafe fn setAcceptsMouseMovedEvents_(self, acceptMouseMovedEvents: BOOL) {
        msg_send![self, setAcceptsMouseMovedEvents:acceptMouseMovedEvents]
    }

    unsafe fn windowNumberAtPoint_belowWindowWithWindowNumber_(self,
                                                               point: NSPoint,
                                                               windowNumber: NSInteger) -> NSInteger {
        msg_send![self, windowNumberAtPoint:point belowWindowWithWindowNumber:windowNumber]
    }

    // Converting Coordinates

    unsafe fn backingScaleFactor(self) -> CGFloat {
        msg_send![self, backingScaleFactor]
    }

    unsafe fn backingAlignedRect_options_(self, rect: NSRect, options: NSAlignmentOptions) -> NSRect {
        msg_send![self, backingAlignedRect:rect options:options]
    }

    unsafe fn convertRectFromBacking_(self, rect: NSRect) -> NSRect {
        msg_send![self, convertRectFromBacking:rect]
    }

    unsafe fn convertRectToBacking_(self, rect: NSRect) -> NSRect {
        msg_send![self, convertRectToBacking:rect]
    }

    unsafe fn convertRectToScreen_(self, rect: NSRect) -> NSRect {
        msg_send![self, convertRectToScreen:rect]
    }

    unsafe fn convertRectFromScreen_(self, rect: NSRect) -> NSRect {
        msg_send![self, convertRectFromScreen:rect]
    }

    // Accessing Edited Status

    unsafe fn setDocumentEdited_(self, documentEdited: BOOL) {
        msg_send![self, setDocumentEdited:documentEdited]
    }

    // Managing Titles

    unsafe fn title(self) -> id {
        msg_send![self, title]
    }

    unsafe fn setTitle_(self, title: id) {
        msg_send![self, setTitle:title]
    }

    unsafe fn setTitleWithRepresentedFilename_(self, filePath: id) {
        msg_send![self, setTitleWithRepresentedFilename:filePath]
    }

    unsafe fn setTitleVisibility_(self, visibility: NSWindowTitleVisibility) {
        msg_send![self, setTitleVisibility:visibility]
    }

    unsafe fn setTitlebarAppearsTransparent_(self, transparent: BOOL) {
        msg_send![self, setTitlebarAppearsTransparent:transparent]
    }

    unsafe fn representedFilename(self) -> id {
        msg_send![self, representedFilename]
    }

    unsafe fn setRepresentedFilename_(self, filePath: id) {
        msg_send![self, setRepresentedFilename:filePath]
    }

    unsafe fn representedURL(self) -> id {
        msg_send![self, representedURL]
    }

    unsafe fn setRepresentedURL_(self, representedURL: id) {
        msg_send![self, setRepresentedURL:representedURL]
    }

    // Accessing Screen Information

    unsafe fn screen(self) -> id {
        msg_send![self, screen]
    }

    unsafe fn deepestScreen(self) -> id {
        msg_send![self, deepestScreen]
    }

    unsafe fn displaysWhenScreenProfileChanges(self) -> BOOL {
        msg_send![self, displaysWhenScreenProfileChanges]
    }

    unsafe fn setDisplaysWhenScreenProfileChanges_(self, displaysWhenScreenProfileChanges: BOOL) {
        msg_send![self, setDisplaysWhenScreenProfileChanges:displaysWhenScreenProfileChanges]
    }

    // Moving Windows

    unsafe fn setMovableByWindowBackground_(self, movableByWindowBackground: BOOL) {
        msg_send![self, setMovableByWindowBackground:movableByWindowBackground]
    }

    unsafe fn setMovable_(self, movable: BOOL) {
        msg_send![self, setMovable:movable]
    }

    unsafe fn center(self) {
        msg_send![self, center]
    }

    // Closing Windows

    unsafe fn performClose_(self, sender: id) {
        msg_send![self, performClose:sender]
    }

    unsafe fn close(self) {
        msg_send![self, close]
    }

    unsafe fn setReleasedWhenClosed_(self, releasedWhenClosed: BOOL) {
        msg_send![self, setReleasedWhenClosed:releasedWhenClosed]
    }

    // Minimizing Windows

    unsafe fn performMiniaturize_(self, sender: id) {
        msg_send![self, performMiniaturize:sender]
    }

    unsafe fn miniaturize_(self, sender: id) {
        msg_send![self, miniaturize:sender]
    }

    unsafe fn deminiaturize_(self, sender: id) {
        msg_send![self, deminiaturize:sender]
    }

    unsafe fn miniwindowImage(self) -> id {
        msg_send![self, miniwindowImage]
    }

    unsafe fn setMiniwindowImage_(self, miniwindowImage: id) {
        msg_send![self, setMiniwindowImage:miniwindowImage]
    }

    unsafe fn miniwindowTitle(self) -> id {
        msg_send![self, miniwindowTitle]
    }

    unsafe fn setMiniwindowTitle_(self, miniwindowTitle: id) {
        msg_send![self, setMiniwindowTitle:miniwindowTitle]
    }

    // TODO: Getting the Dock Tile
    // TODO: Printing Windows
    // TODO: Providing Services
    // TODO: Working with Carbon
    // TODO: Triggering Constraint-Based Layout
    // TODO: Debugging Constraint-Based Layout
    // TODO: Constraint-Based Layouts
}

pub trait NSView {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class("NSView"), alloc]
    }

    unsafe fn init(self) -> id;
    unsafe fn initWithFrame_(self, frameRect: NSRect) -> id;
    unsafe fn bounds(self) -> NSRect;
    unsafe fn frame(self) -> NSRect;
    unsafe fn display_(self);
    unsafe fn setWantsBestResolutionOpenGLSurface_(self, flag: BOOL);
    unsafe fn convertPoint_fromView_(self, point: NSPoint, view: id) -> NSPoint;
    unsafe fn addSubview_(self, view: id);
    unsafe fn superview(self) -> id;
    unsafe fn removeFromSuperview(self);
    
    unsafe fn wantsLayer(self) -> BOOL;
    unsafe fn setWantsLayer(self, wantsLayer: BOOL);
    unsafe fn layer(self) -> id;
    unsafe fn setLayer(self, layer: id);
}

impl NSView for id {
    unsafe fn init(self) -> id {
        msg_send![self, init]
    }

    unsafe fn initWithFrame_(self, frameRect: NSRect) -> id {
        msg_send![self, initWithFrame:frameRect]
    }

    unsafe fn bounds(self) -> NSRect {
        msg_send![self, bounds]
    }

    unsafe fn frame(self) -> NSRect {
        msg_send![self, frame]
    }

    unsafe fn display_(self) {
        msg_send![self, display]
    }

    unsafe fn setWantsBestResolutionOpenGLSurface_(self, flag: BOOL) {
        msg_send![self, setWantsBestResolutionOpenGLSurface:flag]
    }

    unsafe fn convertPoint_fromView_(self, point: NSPoint, view: id) -> NSPoint {
        msg_send![self, convertPoint:point fromView:view]
    }

    unsafe fn addSubview_(self, view: id) {
        msg_send![self, addSubview:view]
    }

    unsafe fn superview(self) -> id {
        msg_send![self, superview]
    }

    unsafe fn removeFromSuperview(self) {
        msg_send![self, removeFromSuperview]
    }

    unsafe fn wantsLayer(self) -> BOOL {
        msg_send![self, wantsLayer]
    }

    unsafe fn setWantsLayer(self, wantsLayer: BOOL) {
        msg_send![self, setWantsLayer:wantsLayer]
    }

    unsafe fn layer(self) -> id {
        msg_send![self, layer]
    }

    unsafe fn setLayer(self, layer: id) {
        msg_send![self, setLayer:layer]
    }
}

pub trait NSOpenGLView {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class("NSOpenGLView"), alloc]
    }

    unsafe fn initWithFrame_pixelFormat_(self, frameRect: NSRect, format: id) -> id;
    unsafe fn display_(self);
    unsafe fn setOpenGLContext_(self, context: id);
    unsafe fn setPixelFormat_(self, pixelformat: id);
}

impl NSOpenGLView for id {
    unsafe fn initWithFrame_pixelFormat_(self,  frameRect: NSRect, format: id) -> id {
        msg_send![self, initWithFrame:frameRect pixelFormat:format]
    }

    unsafe fn display_(self) {
        msg_send![self, display]
    }

    unsafe fn setOpenGLContext_(self, context: id) {
        msg_send![self, setOpenGLContext:context]
    }

    unsafe fn setPixelFormat_(self, pixelformat: id) {
        msg_send![self, setPixelFormat:pixelformat]
    }
}

pub trait NSOpenGLPixelFormat {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class("NSOpenGLPixelFormat"), alloc]
    }

    // Creating an NSOpenGLPixelFormat Object

    unsafe fn initWithAttributes_(self, attributes: &[u32]) -> id;

    // Managing the Pixel Format

    unsafe fn getValues_forAttribute_forVirtualScreen_(self, val: *mut GLint, attrib: NSOpenGLPixelFormatAttribute, screen: GLint);
    unsafe fn numberOfVirtualScreens(self) -> GLint;

}

impl NSOpenGLPixelFormat for id {
    // Creating an NSOpenGLPixelFormat Object

    unsafe fn initWithAttributes_(self, attributes: &[u32]) -> id {
        msg_send![self, initWithAttributes:attributes]
    }

    // Managing the Pixel Format

    unsafe fn getValues_forAttribute_forVirtualScreen_(self, val: *mut GLint, attrib: NSOpenGLPixelFormatAttribute, screen: GLint) {
        msg_send![self, getValues:val forAttribute:attrib forVirtualScreen:screen]
    }

    unsafe fn numberOfVirtualScreens(self) -> GLint {
        msg_send![self, numberOfVirtualScreens]
    }
}

pub trait NSOpenGLContext {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class("NSOpenGLContext"), alloc]
    }

    // Context Creation
    unsafe fn initWithFormat_shareContext_(self, format: id /* (NSOpenGLPixelFormat *) */, shareContext: id /* (NSOpenGLContext *) */) -> id /* (instancetype) */;
    unsafe fn initWithCGLContextObj_(self, context: CGLContextObj) -> id /* (instancetype) */;

    // Managing the Current Context
    unsafe fn clearCurrentContext(_: Self);
    unsafe fn currentContext(_: Self) -> id /* (NSOpenGLContext *) */;
    unsafe fn makeCurrentContext(self);

    // Drawable Object Management
    unsafe fn setView_(self, view: id /* (NSView *) */);
    unsafe fn view(self) -> id /* (NSView *) */;
    unsafe fn clearDrawable(self);
    unsafe fn update(self);

    // Flushing the Drawing Buffer
    unsafe fn flushBuffer(self);

    // Context Parameter Handling
    unsafe fn setValues_forParameter_(self, vals: *const GLint, param: NSOpenGLContextParameter);
    unsafe fn getValues_forParameter_(self, vals: *mut GLint, param: NSOpenGLContextParameter);

    // Working with Virtual Screens
    unsafe fn setCurrentVirtualScreen_(self, screen: GLint);
    unsafe fn currentVirtualScreen(self) -> GLint;

    // Getting the CGL Context Object
    unsafe fn CGLContextObj(self) -> CGLContextObj;
}

impl NSOpenGLContext for id {
    // Context Creation

    unsafe fn initWithFormat_shareContext_(self, format: id /* (NSOpenGLPixelFormat *) */, shareContext: id /* (NSOpenGLContext *) */) -> id /* (instancetype) */ {
        msg_send![self, initWithFormat:format shareContext:shareContext]
    }

    unsafe fn initWithCGLContextObj_(self, context: CGLContextObj) -> id /* (instancetype) */ {
        msg_send![self, initWithCGLContextObj:context]
    }

    // Managing the Current Context

    unsafe fn clearCurrentContext(_: Self) {
        msg_send![class("NSOpenGLContext"), clearCurrentContext]
    }

    unsafe fn currentContext(_: Self) -> id /* (NSOpenGLContext *) */ {
        msg_send![class("NSOpenGLContext"), currentContext]
    }

    unsafe fn makeCurrentContext(self) {
        msg_send![self, makeCurrentContext]
    }

    // Drawable Object Management

    unsafe fn setView_(self, view: id /* (NSView *) */) {
        msg_send![self, setView:view]
    }

    unsafe fn view(self) -> id /* (NSView *) */ {
        msg_send![self, view]
    }

    unsafe fn clearDrawable(self) {
        msg_send![self, clearDrawable]
    }

    unsafe fn update(self) {
        msg_send![self, update]
    }

    // Flushing the Drawing Buffer

    unsafe fn flushBuffer(self) {
        msg_send![self, flushBuffer]
    }

    // Context Parameter Handling

    unsafe fn setValues_forParameter_(self, vals: *const GLint, param: NSOpenGLContextParameter) {
        msg_send![self, setValues:vals forParameter:param]
    }

    unsafe fn getValues_forParameter_(self, vals: *mut GLint, param: NSOpenGLContextParameter) {
        msg_send![self, getValues:vals forParameter:param]
    }

    // Working with Virtual Screens

    unsafe fn setCurrentVirtualScreen_(self, screen: GLint) {
        msg_send![self, setCurrentVirtualScreen:screen]
    }

    unsafe fn currentVirtualScreen(self) -> GLint {
        msg_send![self, currentVirtualScreen]
    }

    // Getting the CGL Context Object

    unsafe fn CGLContextObj(self) -> CGLContextObj {
        msg_send![self, CGLContextObj]
    }
}

bitflags! {
    flags NSEventSwipeTrackingOptions: NSUInteger {
        const NSEventSwipeTrackingLockDirection         = 0x1 << 0,
        const NSEventSwipeTrackingClampGestureAmount    = 0x1 << 1,
    }
}

#[repr(i64)] // NSInteger
pub enum NSEventGestureAxis {
    NSEventGestureAxisNone = 0,
    NSEventGestureAxisHorizontal,
    NSEventGestureAxisVertical,
}

bitflags! {
    flags NSEventPhase: NSUInteger {
       const NSEventPhaseNone        = 0,
       const NSEventPhaseBegan       = 0x1 << 0,
       const NSEventPhaseStationary  = 0x1 << 1,
       const NSEventPhaseChanged     = 0x1 << 2,
       const NSEventPhaseEnded       = 0x1 << 3,
       const NSEventPhaseCancelled   = 0x1 << 4,
       const NSEventPhaseMayBegin    = 0x1 << 5,
    }
}

bitflags! {
    flags NSTouchPhase: NSUInteger {
        const NSTouchPhaseBegan         = 1 << 0,
        const NSTouchPhaseMoved         = 1 << 1,
        const NSTouchPhaseStationary    = 1 << 2,
        const NSTouchPhaseEnded         = 1 << 3,
        const NSTouchPhaseCancelled     = 1 << 4,
        const NSTouchPhaseTouching      = NSTouchPhaseBegan.bits
                                        | NSTouchPhaseMoved.bits
                                        | NSTouchPhaseStationary.bits,
        const NSTouchPhaseAny           = !0, // NSUIntegerMax
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u64)] // NSUInteger
pub enum NSEventType {
    NSLeftMouseDown         = 1,
    NSLeftMouseUp           = 2,
    NSRightMouseDown        = 3,
    NSRightMouseUp          = 4,
    NSMouseMoved            = 5,
    NSLeftMouseDragged      = 6,
    NSRightMouseDragged     = 7,
    NSMouseEntered          = 8,
    NSMouseExited           = 9,
    NSKeyDown               = 10,
    NSKeyUp                 = 11,
    NSFlagsChanged          = 12,
    NSAppKitDefined         = 13,
    NSSystemDefined         = 14,
    NSApplicationDefined    = 15,
    NSPeriodic              = 16,
    NSCursorUpdate          = 17,
    NSScrollWheel           = 22,
    NSTabletPoint           = 23,
    NSTabletProximity       = 24,
    NSOtherMouseDown        = 25,
    NSOtherMouseUp          = 26,
    NSOtherMouseDragged     = 27,
    NSEventTypeGesture      = 29,
    NSEventTypeMagnify      = 30,
    NSEventTypeSwipe        = 31,
    NSEventTypeRotate       = 18,
    NSEventTypeBeginGesture = 19,
    NSEventTypeEndGesture   = 20,
    NSEventTypePressure     = 34,
}

bitflags! {
    flags NSEventMask: libc::c_ulonglong {
        const NSLeftMouseDownMask         = 1 << NSLeftMouseDown as libc::c_ulonglong,
        const NSLeftMouseUpMask           = 1 << NSLeftMouseUp as libc::c_ulonglong,
        const NSRightMouseDownMask        = 1 << NSRightMouseDown as libc::c_ulonglong,
        const NSRightMouseUpMask          = 1 << NSRightMouseUp as libc::c_ulonglong,
        const NSMouseMovedMask            = 1 << NSMouseMoved as libc::c_ulonglong,
        const NSLeftMouseDraggedMask      = 1 << NSLeftMouseDragged as libc::c_ulonglong,
        const NSRightMouseDraggedMask     = 1 << NSRightMouseDragged as libc::c_ulonglong,
        const NSMouseEnteredMask          = 1 << NSMouseEntered as libc::c_ulonglong,
        const NSMouseExitedMask           = 1 << NSMouseExited as libc::c_ulonglong,
        const NSKeyDownMask               = 1 << NSKeyDown as libc::c_ulonglong,
        const NSKeyUpMask                 = 1 << NSKeyUp as libc::c_ulonglong,
        const NSFlagsChangedMask          = 1 << NSFlagsChanged as libc::c_ulonglong,
        const NSAppKitDefinedMask         = 1 << NSAppKitDefined as libc::c_ulonglong,
        const NSSystemDefinedMask         = 1 << NSSystemDefined as libc::c_ulonglong,
        const NSApplicationDefinedMask    = 1 << NSApplicationDefined as libc::c_ulonglong,
        const NSPeriodicMask              = 1 << NSPeriodic as libc::c_ulonglong,
        const NSCursorUpdateMask          = 1 << NSCursorUpdate as libc::c_ulonglong,
        const NSScrollWheelMask           = 1 << NSScrollWheel as libc::c_ulonglong,
        const NSTabletPointMask           = 1 << NSTabletPoint as libc::c_ulonglong,
        const NSTabletProximityMask       = 1 << NSTabletProximity as libc::c_ulonglong,
        const NSOtherMouseDownMask        = 1 << NSOtherMouseDown as libc::c_ulonglong,
        const NSOtherMouseUpMask          = 1 << NSOtherMouseUp as libc::c_ulonglong,
        const NSOtherMouseDraggedMask     = 1 << NSOtherMouseDragged as libc::c_ulonglong,
        const NSEventMaskGesture          = 1 << NSEventTypeGesture as libc::c_ulonglong,
        const NSEventMaskSwipe            = 1 << NSEventTypeSwipe as libc::c_ulonglong,
        const NSEventMaskRotate           = 1 << NSEventTypeRotate as libc::c_ulonglong,
        const NSEventMaskBeginGesture     = 1 << NSEventTypeBeginGesture as libc::c_ulonglong,
        const NSEventMaskEndGesture       = 1 << NSEventTypeEndGesture as libc::c_ulonglong,
        const NSEventMaskPressure         = 1 << NSEventTypePressure as libc::c_ulonglong,
        const NSAnyEventMask              = 0xffffffff,
    }
}

impl NSEventMask {
    pub fn from_type(ty: NSEventType) -> NSEventMask {
        NSEventMask { bits: 1 << ty as libc::c_ulonglong }
    }
}

bitflags! {
    flags NSEventModifierFlags: NSUInteger {
        const NSAlphaShiftKeyMask                     = 1 << 16,
        const NSShiftKeyMask                          = 1 << 17,
        const NSControlKeyMask                        = 1 << 18,
        const NSAlternateKeyMask                      = 1 << 19,
        const NSCommandKeyMask                        = 1 << 20,
        const NSNumericPadKeyMask                     = 1 << 21,
        const NSHelpKeyMask                           = 1 << 22,
        const NSFunctionKeyMask                       = 1 << 23,
        const NSDeviceIndependentModifierFlagsMask    = 0xffff0000,
    }
}

// Not sure of the type here
pub enum NSPointingDeviceType {
    // TODO: Not sure what these values are
    // NSUnknownPointingDevice = NX_TABLET_POINTER_UNKNOWN,
    // NSPenPointingDevice     = NX_TABLET_POINTER_PEN,
    // NSCursorPointingDevice  = NX_TABLET_POINTER_CURSOR,
    // NSEraserPointingDevice  = NX_TABLET_POINTER_ERASER,
}

// Not sure of the type here
pub enum NSEventButtonMask {
    // TODO: Not sure what these values are
    // NSPenTipMask =       NX_TABLET_BUTTON_PENTIPMASK,
    // NSPenLowerSideMask = NX_TABLET_BUTTON_PENLOWERSIDEMASK,
    // NSPenUpperSideMask = NX_TABLET_BUTTON_PENUPPERSIDEMASK,
}

#[repr(i16)]
pub enum NSEventSubtype {
    // TODO: Not sure what these values are
    // NSMouseEventSubtype           = NX_SUBTYPE_DEFAULT,
    // NSTabletPointEventSubtype     = NX_SUBTYPE_TABLET_POINT,
    // NSTabletProximityEventSubtype = NX_SUBTYPE_TABLET_PROXIMITY
    // NSTouchEventSubtype           = NX_SUBTYPE_MOUSE_TOUCH,
    NSWindowExposedEventType = 0,
    NSApplicationActivatedEventType = 1,
    NSApplicationDeactivatedEventType = 2,
    NSWindowMovedEventType = 4,
    NSScreenChangedEventType = 8,
    NSAWTEventType = 16,
}

pub const NSUpArrowFunctionKey: libc::c_ushort = 0xF700;
pub const NSDownArrowFunctionKey: libc::c_ushort = 0xF701;
pub const NSLeftArrowFunctionKey: libc::c_ushort = 0xF702;
pub const NSRightArrowFunctionKey: libc::c_ushort = 0xF703;
pub const NSF1FunctionKey: libc::c_ushort = 0xF704;
pub const NSF2FunctionKey: libc::c_ushort = 0xF705;
pub const NSF3FunctionKey: libc::c_ushort = 0xF706;
pub const NSF4FunctionKey: libc::c_ushort = 0xF707;
pub const NSF5FunctionKey: libc::c_ushort = 0xF708;
pub const NSF6FunctionKey: libc::c_ushort = 0xF709;
pub const NSF7FunctionKey: libc::c_ushort = 0xF70A;
pub const NSF8FunctionKey: libc::c_ushort = 0xF70B;
pub const NSF9FunctionKey: libc::c_ushort = 0xF70C;
pub const NSF10FunctionKey: libc::c_ushort = 0xF70D;
pub const NSF11FunctionKey: libc::c_ushort = 0xF70E;
pub const NSF12FunctionKey: libc::c_ushort = 0xF70F;
pub const NSF13FunctionKey: libc::c_ushort = 0xF710;
pub const NSF14FunctionKey: libc::c_ushort = 0xF711;
pub const NSF15FunctionKey: libc::c_ushort = 0xF712;
pub const NSF16FunctionKey: libc::c_ushort = 0xF713;
pub const NSF17FunctionKey: libc::c_ushort = 0xF714;
pub const NSF18FunctionKey: libc::c_ushort = 0xF715;
pub const NSF19FunctionKey: libc::c_ushort = 0xF716;
pub const NSF20FunctionKey: libc::c_ushort = 0xF717;
pub const NSF21FunctionKey: libc::c_ushort = 0xF718;
pub const NSF22FunctionKey: libc::c_ushort = 0xF719;
pub const NSF23FunctionKey: libc::c_ushort = 0xF71A;
pub const NSF24FunctionKey: libc::c_ushort = 0xF71B;
pub const NSF25FunctionKey: libc::c_ushort = 0xF71C;
pub const NSF26FunctionKey: libc::c_ushort = 0xF71D;
pub const NSF27FunctionKey: libc::c_ushort = 0xF71E;
pub const NSF28FunctionKey: libc::c_ushort = 0xF71F;
pub const NSF29FunctionKey: libc::c_ushort = 0xF720;
pub const NSF30FunctionKey: libc::c_ushort = 0xF721;
pub const NSF31FunctionKey: libc::c_ushort = 0xF722;
pub const NSF32FunctionKey: libc::c_ushort = 0xF723;
pub const NSF33FunctionKey: libc::c_ushort = 0xF724;
pub const NSF34FunctionKey: libc::c_ushort = 0xF725;
pub const NSF35FunctionKey: libc::c_ushort = 0xF726;
pub const NSInsertFunctionKey: libc::c_ushort = 0xF727;
pub const NSDeleteFunctionKey: libc::c_ushort = 0xF728;
pub const NSHomeFunctionKey: libc::c_ushort = 0xF729;
pub const NSBeginFunctionKey: libc::c_ushort = 0xF72A;
pub const NSEndFunctionKey: libc::c_ushort = 0xF72B;
pub const NSPageUpFunctionKey: libc::c_ushort = 0xF72C;
pub const NSPageDownFunctionKey: libc::c_ushort = 0xF72D;
pub const NSPrintScreenFunctionKey: libc::c_ushort = 0xF72E;
pub const NSScrollLockFunctionKey: libc::c_ushort = 0xF72F;
pub const NSPauseFunctionKey: libc::c_ushort = 0xF730;
pub const NSSysReqFunctionKey: libc::c_ushort = 0xF731;
pub const NSBreakFunctionKey: libc::c_ushort = 0xF732;
pub const NSResetFunctionKey: libc::c_ushort = 0xF733;
pub const NSStopFunctionKey: libc::c_ushort = 0xF734;
pub const NSMenuFunctionKey: libc::c_ushort = 0xF735;
pub const NSUserFunctionKey: libc::c_ushort = 0xF736;
pub const NSSystemFunctionKey: libc::c_ushort = 0xF737;
pub const NSPrintFunctionKey: libc::c_ushort = 0xF738;
pub const NSClearLineFunctionKey: libc::c_ushort = 0xF739;
pub const NSClearDisplayFunctionKey: libc::c_ushort = 0xF73A;
pub const NSInsertLineFunctionKey: libc::c_ushort = 0xF73B;
pub const NSDeleteLineFunctionKey: libc::c_ushort = 0xF73C;
pub const NSInsertCharFunctionKey: libc::c_ushort = 0xF73D;
pub const NSDeleteCharFunctionKey: libc::c_ushort = 0xF73E;
pub const NSPrevFunctionKey: libc::c_ushort = 0xF73F;
pub const NSNextFunctionKey: libc::c_ushort = 0xF740;
pub const NSSelectFunctionKey: libc::c_ushort = 0xF741;
pub const NSExecuteFunctionKey: libc::c_ushort = 0xF742;
pub const NSUndoFunctionKey: libc::c_ushort = 0xF743;
pub const NSRedoFunctionKey: libc::c_ushort = 0xF744;
pub const NSFindFunctionKey: libc::c_ushort = 0xF745;
pub const NSHelpFunctionKey: libc::c_ushort = 0xF746;
pub const NSModeSwitchFunctionKey: libc::c_ushort = 0xF747;

pub trait NSEvent {
    // Creating Events
    unsafe fn keyEventWithType_location_modifierFlags_timestamp_windowNumber_context_characters_charactersIgnoringModifiers_isARepeat_keyCode_(
        _: Self,
        eventType: NSEventType,
        location: NSPoint,
        modifierFlags: NSEventModifierFlags,
        timestamp: NSTimeInterval,
        windowNumber: NSInteger,
        context: id /* (NSGraphicsContext *) */,
        characters: id /* (NSString *) */,
        unmodCharacters: id /* (NSString *) */,
        repeatKey: BOOL,
        code: libc::c_ushort) -> id /* (NSEvent *) */;
    unsafe fn mouseEventWithType_location_modifierFlags_timestamp_windowNumber_context_eventNumber_clickCount_pressure_(
        _: Self,
        eventType: NSEventType,
        location: NSPoint,
        modifierFlags: NSEventModifierFlags,
        timestamp: NSTimeInterval,
        windowNumber: NSInteger,
        context: id /* (NSGraphicsContext *) */,
        eventNumber: NSInteger,
        clickCount: NSInteger,
        pressure: libc::c_float) -> id /* (NSEvent *) */;
    unsafe fn enterExitEventWithType_location_modifierFlags_timestamp_windowNumber_context_eventNumber_trackingNumber_userData_(
        _: Self,
        eventType: NSEventType,
        location: NSPoint,
        modifierFlags: NSEventModifierFlags,
        timestamp: NSTimeInterval,
        windowNumber: NSInteger,
        context: id /* (NSGraphicsContext *) */,
        eventNumber: NSInteger,
        trackingNumber: NSInteger,
        userData: *mut libc::c_void) -> id /* (NSEvent *) */;
    unsafe fn otherEventWithType_location_modifierFlags_timestamp_windowNumber_context_subtype_data1_data2_(
        _: Self,
        eventType: NSEventType,
        location: NSPoint,
        modifierFlags: NSEventModifierFlags,
        timestamp: NSTimeInterval,
        windowNumber: NSInteger,
        context: id /* (NSGraphicsContext *) */,
        subtype: NSEventSubtype,
        data1: NSInteger,
        data2: NSInteger) -> id /* (NSEvent *) */;
    unsafe fn eventWithEventRef_(_: Self, eventRef: *const libc::c_void) -> id;
    unsafe fn eventWithCGEvent_(_: Self, cgEvent: *mut libc::c_void /* CGEventRef */) -> id;

    // Getting General Event Information
    unsafe fn context(self) -> id /* (NSGraphicsContext *) */;
    unsafe fn locationInWindow(self) -> NSPoint;
    unsafe fn modifierFlags(self) -> NSEventModifierFlags;
    unsafe fn timestamp(self) -> NSTimeInterval;
    // NOTE: renamed from `- type` due to Rust keyword collision
    unsafe fn eventType(self) -> NSEventType;
    unsafe fn window(self) -> id /* (NSWindow *) */;
    unsafe fn windowNumber(self) -> NSInteger;
    unsafe fn eventRef(self) -> *const libc::c_void;
    unsafe fn CGEvent(self) -> *mut libc::c_void /* CGEventRef */;

    // Getting Key Event Information
    // NOTE: renamed from `+ modifierFlags` due to conflict with `- modifierFlags`
    unsafe fn currentModifierFlags(_: Self) -> NSEventModifierFlags;
    unsafe fn keyRepeatDelay(_: Self) -> NSTimeInterval;
    unsafe fn keyRepeatInterval(_: Self) -> NSTimeInterval;
    unsafe fn characters(self) -> id /* (NSString *) */;
    unsafe fn charactersIgnoringModifiers(self) -> id /* (NSString *) */;
    unsafe fn keyCode(self) -> libc::c_ushort;

    // Getting Mouse Event Information
    unsafe fn pressedMouseButtons(_: Self) -> NSUInteger;
    unsafe fn doubleClickInterval(_: Self) -> NSTimeInterval;
    unsafe fn mouseLocation(_: Self) -> NSPoint;
    unsafe fn buttonNumber(self) -> NSInteger;
    unsafe fn clickCount(self) -> NSInteger;
    unsafe fn pressure(self) -> libc::c_float;
    unsafe fn stage(self) -> NSInteger;
    unsafe fn setMouseCoalescingEnabled_(_: Self, flag: BOOL);
    unsafe fn isMouseCoalescingEnabled(_: Self) -> BOOL;

    // Getting Mouse-Tracking Event Information
    unsafe fn eventNumber(self) -> NSInteger;
    unsafe fn trackingNumber(self) -> NSInteger;
    unsafe fn trackingArea(self) -> id /* (NSTrackingArea *) */;
    unsafe fn userData(self) -> *const libc::c_void;

    // Getting Custom Event Information
    unsafe fn data1(self) -> NSInteger;
    unsafe fn data2(self) -> NSInteger;
    unsafe fn subtype(self) -> NSEventSubtype;

    // Getting Scroll Wheel Event Information
    unsafe fn deltaX(self) -> CGFloat;
    unsafe fn deltaY(self) -> CGFloat;
    unsafe fn deltaZ(self) -> CGFloat;

    // Getting Tablet Proximity Information
    unsafe fn capabilityMask(self) -> NSUInteger;
    unsafe fn deviceID(self) -> NSUInteger;
    unsafe fn pointingDeviceID(self) -> NSUInteger;
    unsafe fn pointingDeviceSerialNumber(self) -> NSUInteger;
    unsafe fn pointingDeviceType(self) -> NSPointingDeviceType;
    unsafe fn systemTabletID(self) -> NSUInteger;
    unsafe fn tabletID(self) -> NSUInteger;
    unsafe fn uniqueID(self) -> libc::c_ulonglong;
    unsafe fn vendorID(self) -> NSUInteger;
    unsafe fn vendorPointingDeviceType(self) -> NSUInteger;

    // Getting Tablet Pointing Information
    unsafe fn absoluteX(self) -> NSInteger;
    unsafe fn absoluteY(self) -> NSInteger;
    unsafe fn absoluteZ(self) -> NSInteger;
    unsafe fn buttonMask(self) -> NSEventButtonMask;
    unsafe fn rotation(self) -> libc::c_float;
    unsafe fn tangentialPressure(self) -> libc::c_float;
    unsafe fn tilt(self) -> NSPoint;
    unsafe fn vendorDefined(self) -> id;

    // Requesting and Stopping Periodic Events
    unsafe fn startPeriodicEventsAfterDelay_withPeriod_(_: Self, delaySeconds: NSTimeInterval, periodSeconds: NSTimeInterval);
    unsafe fn stopPeriodicEvents(_: Self);

    // Getting Touch and Gesture Information
    unsafe fn magnification(self) -> CGFloat;
    unsafe fn touchesMatchingPhase_inView_(self, phase: NSTouchPhase, view: id /* (NSView *) */) -> id /* (NSSet *) */;
    unsafe fn isSwipeTrackingFromScrollEventsEnabled(_: Self) -> BOOL;

    // Monitoring Application Events
    // TODO: addGlobalMonitorForEventsMatchingMask_handler_ (unsure how to bind to blocks)
    // TODO: addLocalMonitorForEventsMatchingMask_handler_ (unsure how to bind to blocks)
    unsafe fn removeMonitor_(_: Self, eventMonitor: id);

    // Scroll Wheel and Flick Events
    unsafe fn hasPreciseScrollingDeltas(self) -> BOOL;
    unsafe fn scrollingDeltaX(self) -> CGFloat;
    unsafe fn scrollingDeltaY(self) -> CGFloat;
    unsafe fn momentumPhase(self) -> NSEventPhase;
    unsafe fn phase(self) -> NSEventPhase;
    // TODO: trackSwipeEventWithOptions_dampenAmountThresholdMin_max_usingHandler_ (unsure how to bind to blocks)

    // Converting a Mouse Event’s Position into a Sprite Kit Node’s Coordinate Space
    unsafe fn locationInNode_(self, node: id /* (SKNode *) */) -> CGPoint;
}

impl NSEvent for id {
    // Creating Events

    unsafe fn keyEventWithType_location_modifierFlags_timestamp_windowNumber_context_characters_charactersIgnoringModifiers_isARepeat_keyCode_(
        _: Self,
        eventType: NSEventType,
        location: NSPoint,
        modifierFlags: NSEventModifierFlags,
        timestamp: NSTimeInterval,
        windowNumber: NSInteger,
        context: id /* (NSGraphicsContext *) */,
        characters: id /* (NSString *) */,
        unmodCharacters: id /* (NSString *) */,
        repeatKey: BOOL,
        code: libc::c_ushort) -> id /* (NSEvent *) */
    {
        msg_send![class("NSEvent"), keyEventWithType:eventType
                                            location:location
                                       modifierFlags:modifierFlags
                                           timestamp:timestamp
                                        windowNumber:windowNumber
                                             context:context
                                          characters:characters
                         charactersIgnoringModifiers:unmodCharacters
                                           isARepeat:repeatKey
                                             keyCode:code]
    }

    unsafe fn mouseEventWithType_location_modifierFlags_timestamp_windowNumber_context_eventNumber_clickCount_pressure_(
        _: Self,
        eventType: NSEventType,
        location: NSPoint,
        modifierFlags: NSEventModifierFlags,
        timestamp: NSTimeInterval,
        windowNumber: NSInteger,
        context: id /* (NSGraphicsContext *) */,
        eventNumber: NSInteger,
        clickCount: NSInteger,
        pressure: libc::c_float) -> id /* (NSEvent *) */
    {
        msg_send![class("NSEvent"), mouseEventWithType:eventType
                                              location:location
                                         modifierFlags:modifierFlags
                                             timestamp:timestamp
                                          windowNumber:windowNumber
                                               context:context
                                           eventNumber:eventNumber
                                            clickCount:clickCount
                                              pressure:pressure]
    }

    unsafe fn enterExitEventWithType_location_modifierFlags_timestamp_windowNumber_context_eventNumber_trackingNumber_userData_(
        _: Self,
        eventType: NSEventType,
        location: NSPoint,
        modifierFlags: NSEventModifierFlags,
        timestamp: NSTimeInterval,
        windowNumber: NSInteger,
        context: id /* (NSGraphicsContext *) */,
        eventNumber: NSInteger,
        trackingNumber: NSInteger,
        userData: *mut libc::c_void) -> id /* (NSEvent *) */
    {
        msg_send![class("NSEvent"), enterExitEventWithType:eventType
                                                  location:location
                                             modifierFlags:modifierFlags
                                                 timestamp:timestamp
                                              windowNumber:windowNumber
                                                   context:context
                                               eventNumber:eventNumber
                                            trackingNumber:trackingNumber
                                                  userData:userData]
    }

    unsafe fn otherEventWithType_location_modifierFlags_timestamp_windowNumber_context_subtype_data1_data2_(
        _: Self,
        eventType: NSEventType,
        location: NSPoint,
        modifierFlags: NSEventModifierFlags,
        timestamp: NSTimeInterval,
        windowNumber: NSInteger,
        context: id /* (NSGraphicsContext *) */,
        subtype: NSEventSubtype,
        data1: NSInteger,
        data2: NSInteger) -> id /* (NSEvent *) */
    {
        msg_send![class("NSEvent"), otherEventWithType:eventType
                                              location:location
                                         modifierFlags:modifierFlags
                                             timestamp:timestamp
                                          windowNumber:windowNumber
                                               context:context
                                               subtype:subtype
                                                 data1:data1
                                                 data2:data2]
    }

    unsafe fn eventWithEventRef_(_: Self, eventRef: *const libc::c_void) -> id {
        msg_send![class("NSEvent"), eventWithEventRef:eventRef]
    }

    unsafe fn eventWithCGEvent_(_: Self, cgEvent: *mut libc::c_void /* CGEventRef */) -> id {
        msg_send![class("NSEvent"), eventWithCGEvent:cgEvent]
    }

    // Getting General Event Information

    unsafe fn context(self) -> id /* (NSGraphicsContext *) */ {
        msg_send![self, context]
    }

    unsafe fn locationInWindow(self) -> NSPoint {
        msg_send![self, locationInWindow]
    }

    unsafe fn modifierFlags(self) -> NSEventModifierFlags {
        msg_send![self, modifierFlags]
    }

    unsafe fn timestamp(self) -> NSTimeInterval {
        msg_send![self, timestamp]
    }
    // NOTE: renamed from `- type` due to Rust keyword collision

    unsafe fn eventType(self) -> NSEventType {
        msg_send![self, type]
    }

    unsafe fn window(self) -> id /* (NSWindow *) */ {
        msg_send![self, window]
    }

    unsafe fn windowNumber(self) -> NSInteger {
        msg_send![self, windowNumber]
    }

    unsafe fn eventRef(self) -> *const libc::c_void {
        msg_send![self, eventRef]
    }

    unsafe fn CGEvent(self) -> *mut libc::c_void /* CGEventRef */ {
        msg_send![self, CGEvent]
    }

    // Getting Key Event Information

    // NOTE: renamed from `+ modifierFlags` due to conflict with `- modifierFlags`

    unsafe fn currentModifierFlags(_: Self) -> NSEventModifierFlags {
        msg_send![class("NSEvent"), currentModifierFlags]
    }

    unsafe fn keyRepeatDelay(_: Self) -> NSTimeInterval {
        msg_send![class("NSEvent"), keyRepeatDelay]
    }

    unsafe fn keyRepeatInterval(_: Self) -> NSTimeInterval {
        msg_send![class("NSEvent"), keyRepeatInterval]
    }

    unsafe fn characters(self) -> id /* (NSString *) */ {
        msg_send![self, characters]
    }

    unsafe fn charactersIgnoringModifiers(self) -> id /* (NSString *) */ {
        msg_send![self, charactersIgnoringModifiers]
    }

    unsafe fn keyCode(self) -> libc::c_ushort {
        msg_send![self, keyCode]
    }

    // Getting Mouse Event Information

    unsafe fn pressedMouseButtons(_: Self) -> NSUInteger {
        msg_send![class("NSEvent"), pressedMouseButtons]
    }

    unsafe fn doubleClickInterval(_: Self) -> NSTimeInterval {
        msg_send![class("NSEvent"), doubleClickInterval]
    }

    unsafe fn mouseLocation(_: Self) -> NSPoint {
        msg_send![class("NSEvent"), mouseLocation]
    }

    unsafe fn buttonNumber(self) -> NSInteger {
        msg_send![self, buttonNumber]
    }

    unsafe fn clickCount(self) -> NSInteger {
        msg_send![self, clickCount]
    }

    unsafe fn pressure(self) -> libc::c_float {
        msg_send![self, pressure]
    }

    unsafe fn stage(self) -> NSInteger{
        msg_send![self, stage]
    }

    unsafe fn setMouseCoalescingEnabled_(_: Self, flag: BOOL) {
        msg_send![class("NSEvent"), setMouseCoalescingEnabled:flag]
    }

    unsafe fn isMouseCoalescingEnabled(_: Self) -> BOOL {
        msg_send![class("NSEvent"), isMouseCoalescingEnabled]
    }

    // Getting Mouse-Tracking Event Information

    unsafe fn eventNumber(self) -> NSInteger {
        msg_send![self, eventNumber]
    }

    unsafe fn trackingNumber(self) -> NSInteger {
        msg_send![self, trackingNumber]
    }

    unsafe fn trackingArea(self) -> id /* (NSTrackingArea *) */ {
        msg_send![self, trackingArea]
    }

    unsafe fn userData(self) -> *const libc::c_void {
        msg_send![self, userData]
    }

    // Getting Custom Event Information

    unsafe fn data1(self) -> NSInteger {
        msg_send![self, data1]
    }

    unsafe fn data2(self) -> NSInteger {
        msg_send![self, data2]
    }

    unsafe fn subtype(self) -> NSEventSubtype {
        msg_send![self, subtype]
    }

    // Getting Scroll Wheel Event Information

    unsafe fn deltaX(self) -> CGFloat {
        msg_send![self, deltaX]
    }

    unsafe fn deltaY(self) -> CGFloat {
        msg_send![self, deltaY]
    }

    unsafe fn deltaZ(self) -> CGFloat {
        msg_send![self, deltaZ]
    }

    // Getting Tablet Proximity Information

    unsafe fn capabilityMask(self) -> NSUInteger {
        msg_send![self, capabilityMask]
    }

    unsafe fn deviceID(self) -> NSUInteger {
        msg_send![self, deviceID]
    }

    unsafe fn pointingDeviceID(self) -> NSUInteger {
        msg_send![self, pointingDeviceID]
    }

    unsafe fn pointingDeviceSerialNumber(self) -> NSUInteger {
        msg_send![self, pointingDeviceSerialNumber]
    }

    unsafe fn pointingDeviceType(self) -> NSPointingDeviceType {
        msg_send![self, pointingDeviceType]
    }

    unsafe fn systemTabletID(self) -> NSUInteger {
        msg_send![self, systemTabletID]
    }

    unsafe fn tabletID(self) -> NSUInteger {
        msg_send![self, tabletID]
    }

    unsafe fn uniqueID(self) -> libc::c_ulonglong {
        msg_send![self, uniqueID]
    }

    unsafe fn vendorID(self) -> NSUInteger {
        msg_send![self, vendorID]
    }

    unsafe fn vendorPointingDeviceType(self) -> NSUInteger {
        msg_send![self, vendorPointingDeviceType]
    }

    // Getting Tablet Pointing Information

    unsafe fn absoluteX(self) -> NSInteger {
        msg_send![self, absoluteX]
    }

    unsafe fn absoluteY(self) -> NSInteger {
        msg_send![self, absoluteY]
    }

    unsafe fn absoluteZ(self) -> NSInteger {
        msg_send![self, absoluteZ]
    }

    unsafe fn buttonMask(self) -> NSEventButtonMask {
        msg_send![self, buttonMask]
    }

    unsafe fn rotation(self) -> libc::c_float {
        msg_send![self, rotation]
    }

    unsafe fn tangentialPressure(self) -> libc::c_float {
        msg_send![self, tangentialPressure]
    }

    unsafe fn tilt(self) -> NSPoint {
        msg_send![self, tilt]
    }

    unsafe fn vendorDefined(self) -> id {
        msg_send![self, vendorDefined]
    }

    // Requesting and Stopping Periodic Events

    unsafe fn startPeriodicEventsAfterDelay_withPeriod_(_: Self, delaySeconds: NSTimeInterval, periodSeconds: NSTimeInterval) {
        msg_send![class("NSEvent"), startPeriodicEventsAfterDelay:delaySeconds withPeriod:periodSeconds]
    }

    unsafe fn stopPeriodicEvents(_: Self) {
        msg_send![class("NSEvent"), stopPeriodicEvents]
    }

    // Getting Touch and Gesture Information

    unsafe fn magnification(self) -> CGFloat {
        msg_send![self, magnification]
    }

    unsafe fn touchesMatchingPhase_inView_(self, phase: NSTouchPhase, view: id /* (NSView *) */) -> id /* (NSSet *) */ {
        msg_send![self, touchesMatchingPhase:phase inView:view]
    }

    unsafe fn isSwipeTrackingFromScrollEventsEnabled(_: Self) -> BOOL {
        msg_send![class("NSEvent"), isSwipeTrackingFromScrollEventsEnabled]
    }

    // Monitoring Application Events

    // TODO: addGlobalMonitorForEventsMatchingMask_handler_ (unsure how to bind to blocks)
    // TODO: addLocalMonitorForEventsMatchingMask_handler_ (unsure how to bind to blocks)

    unsafe fn removeMonitor_(_: Self, eventMonitor: id) {
        msg_send![class("NSEvent"), removeMonitor:eventMonitor]
    }

    // Scroll Wheel and Flick Events

    unsafe fn hasPreciseScrollingDeltas(self) -> BOOL {
        msg_send![self, hasPreciseScrollingDeltas]
    }

    unsafe fn scrollingDeltaX(self) -> CGFloat {
        msg_send![self, scrollingDeltaX]
    }

    unsafe fn scrollingDeltaY(self) -> CGFloat {
        msg_send![self, scrollingDeltaY]
    }

    unsafe fn momentumPhase(self) -> NSEventPhase {
        msg_send![self, momentumPhase]
    }

    unsafe fn phase(self) -> NSEventPhase {
        msg_send![self, phase]
    }

    // TODO: trackSwipeEventWithOptions_dampenAmountThresholdMin_max_usingHandler_ (unsure how to bind to blocks)

    // Converting a Mouse Event’s Position into a Sprite Kit Node’s Coordinate Space
    unsafe fn locationInNode_(self, node: id /* (SKNode *) */) -> CGPoint {
        msg_send![self, locationInNode:node]
    }
}

pub trait NSScreen {
    // Getting NSScreen Objects
    unsafe fn mainScreen(_: Self) -> id /* (NSScreen *) */;
    unsafe fn deepestScreen(_: Self) -> id /* (NSScreen *) */;
    unsafe fn screens(_: Self) -> id /* (NSArray *) */;

    // Getting Screen Information
    unsafe fn depth(self) -> NSWindowDepth;
    unsafe fn frame(self) -> NSRect;
    unsafe fn supportedWindowDepths(self) -> *const NSWindowDepth;
    unsafe fn deviceDescription(self) -> id /* (NSDictionary *) */;
    unsafe fn visibleFrame(self) -> NSRect;
    unsafe fn colorSpace(self) -> id /* (NSColorSpace *) */;
    unsafe fn screensHaveSeparateSpaces(_: Self) -> BOOL;

    // Screen Backing Coordinate Conversion
    unsafe fn backingAlignedRect_options_(self, aRect: NSRect, options: NSAlignmentOptions) -> NSRect;
    unsafe fn backingScaleFactor(self) -> CGFloat;
    unsafe fn convertRectFromBacking_(self, aRect: NSRect) -> NSRect;
    unsafe fn convertRectToBacking_(self, aRect: NSRect) -> NSRect;
}

impl NSScreen for id {
    // Getting NSScreen Objects

    unsafe fn mainScreen(_: Self) -> id /* (NSScreen *) */ {
        msg_send![class("NSScreen"), mainScreen]
    }

    unsafe fn deepestScreen(_: Self) -> id /* (NSScreen *) */ {
        msg_send![class("NSScreen"), deepestScreen]
    }

    unsafe fn screens(_: Self) -> id /* (NSArray *) */ {
        msg_send![class("NSScreen"), screens]
    }

    // Getting Screen Information

    unsafe fn depth(self) -> NSWindowDepth {
        msg_send![self, depth]
    }

    unsafe fn frame(self) -> NSRect {
        msg_send![self, frame]
    }

    unsafe fn supportedWindowDepths(self) -> *const NSWindowDepth {
        msg_send![self, supportedWindowDepths]
    }

    unsafe fn deviceDescription(self) -> id /* (NSDictionary *) */ {
        msg_send![self, deviceDescription]
    }

    unsafe fn visibleFrame(self) -> NSRect {
        msg_send![self, visibleFrame]
    }

    unsafe fn colorSpace(self) -> id /* (NSColorSpace *) */ {
        msg_send![self, colorSpace]
    }

    unsafe fn screensHaveSeparateSpaces(_: Self) -> BOOL {
        msg_send![class("NSScreen"), screensHaveSeparateSpaces]
    }

    // Screen Backing Coordinate Conversion

    unsafe fn backingAlignedRect_options_(self, aRect: NSRect, options: NSAlignmentOptions) -> NSRect {
        msg_send![self, backingAlignedRect:aRect options:options]
    }

    unsafe fn backingScaleFactor(self) -> CGFloat {
        msg_send![self, backingScaleFactor]
    }

    unsafe fn convertRectFromBacking_(self, aRect: NSRect) -> NSRect {
        msg_send![self, convertRectFromBacking:aRect]
    }

    unsafe fn convertRectToBacking_(self, aRect: NSRect) -> NSRect {
        msg_send![self, convertRectToBacking:aRect]
    }
}

pub trait NSButton {
     unsafe fn setImage_(self, img: id /* (NSImage *) */);
}

impl NSButton for id {
    unsafe fn setImage_(self, img: id /* (NSImage *) */) {
        msg_send![self, setImage:img]
    }
}

pub trait NSImage {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class("NSImage"), alloc]
    }

    unsafe fn initByReferencingFile_(self, file_name: id /* (NSString *) */) -> id;
    unsafe fn initWithContentsOfFile_(self, file_name: id /* (NSString *) */) -> id;
    unsafe fn name(self) -> id /* (NSString *) */;
    unsafe fn setName_(self, name: id /* (NSString *) */) -> BOOL;
}

impl NSImage for id {
    unsafe fn initByReferencingFile_(self, file_name: id /* (NSString *) */) -> id {
        msg_send![self, initByReferencingFile:file_name]
    }

    unsafe fn initWithContentsOfFile_(self, file_name: id /* (NSString *) */) -> id {
        msg_send![self, initWithContentsOfFile:file_name]
    }

    unsafe fn name(self) -> id /* (NSString *) */ {
        msg_send![self, name]
    }

    unsafe fn setName_(self, name: id /* (NSString *) */) -> BOOL {
        msg_send![self, setName:name]
    }
}

pub const NSVariableStatusItemLength: CGFloat = -1.0;
pub const NSSquareStatusItemLength: CGFloat = -2.0;

pub trait NSStatusItem {
    unsafe fn statusBar(self) -> id /* (NSStatusBar *) */;
    unsafe fn button(self) -> id /* (NSStatusBarButton *) */;
    unsafe fn menu(self) -> id;
    unsafe fn setMenu_(self, menu: id);
    unsafe fn length(self) -> CGFloat;
    unsafe fn setLength_(self, length: CGFloat);
}

impl NSStatusItem for id {
    unsafe fn statusBar(self) -> id /* (NSStatusBar *) */ {
        msg_send![self, statusBar]
    }

    unsafe fn button(self) -> id /* (NSStatusBarButton *) */ {
        msg_send![self, button]
    }

    unsafe fn menu(self) -> id {
        msg_send![self, menu]
    }

    unsafe fn setMenu_(self, menu: id) {
        msg_send![self, setMenu:menu]
    }

    unsafe fn length(self) -> CGFloat {
        msg_send![self, length]
    }

    unsafe fn setLength_(self, length: CGFloat) {
        msg_send![self, setLength: length]
    }
}

pub trait NSStatusBar {
    unsafe fn systemStatusBar(_: Self) -> id {
        msg_send![class("NSStatusBar"), systemStatusBar]
    }

    unsafe fn statusItemWithLength_(self, length: CGFloat) -> id /* (NSStatusItem *) */;
    unsafe fn removeStatusItem_(self, item: id /* (NSStatusItem *) */);
    unsafe fn isVertical(self) -> BOOL;
}

impl NSStatusBar for id {
    unsafe fn statusItemWithLength_(self, length: CGFloat) -> id /* (NSStatusItem *) */ {
        msg_send![self, statusItemWithLength:length]
    }

    unsafe fn removeStatusItem_(self, item: id /* (NSStatusItem *) */) {
        msg_send![self, removeStatusItem:item]
    }

    unsafe fn isVertical(self) -> BOOL {
        msg_send![self, isVertical]
    }
}
