//
//  WBWindow.m
//  WBWindow
//
//  Created by Will Bartlett on 31/01/2017.
//  Copyright © 2017 Will Bartlett. All rights reserved.
//

#import "WBWindow.h"

@implementation WBWindow

-(id)init {
  if (self = [super init]) {
    [self setHasShadow:NO];
    [self setStyleMask:NSWindowStyleMaskBorderless | NSWindowStyleMaskResizable];
  }
  return self;
}

-(BOOL)canBecomeKeyWindow {
    return YES;
}

@end
