//
//  WBWindow.m
//  WBWindow
//
//  Created by Will Bartlett on 31/01/2017.
//  Copyright © 2017 Will Bartlett. All rights reserved.
//

#import "WBWindow.h"

@implementation WBWindow

-(id) initWithContentRect:(NSRect)contentRect
                styleMask:(NSUInteger)style
                  backing:(NSBackingStoreType)bufferingType
                    defer:(BOOL)flag {

  if (self = [super initWithContentRect:contentRect
                              styleMask:style
                                backing:bufferingType
                                  defer:flag]) {
    [self setHasShadow:NO];
    [self setStyleMask:NSBorderlessWindowMask | NSResizableWindowMask];
  }

  return self;
}

-(BOOL) canBecomeKeyWindow {
    return YES;
}

@end
