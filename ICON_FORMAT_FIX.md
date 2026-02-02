# Icon Format Fix - RGB to RGBA Conversion

## Issue

Build was failing with error:
```
error: proc macro panicked
  --> src/lib.rs:101:14
   |
101 |         .run(tauri::generate_context!())
    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: message: icon /Users/nixen/Projects/czytaj/src-tauri/icons/32x32.png is not RGBA
```

## Root Cause

The PNG icon files were created in RGB format (color type 2) without an alpha channel. Tauri requires all icon files to be in RGBA format (color type 6) with an alpha channel.

### Before Fix
```
32x32.png:      PNG image data, 32 x 32, 8-bit/color RGB, non-interlaced
128x128.png:    PNG image data, 128 x 128, 8-bit/color RGB, non-interlaced
128x128@2x.png: PNG image data, 256 x 256, 8-bit/color RGB, non-interlaced
```

## Solution

Recreated all PNG icon files with RGBA format (color type 6) including an alpha channel. All pixels now have full opacity (alpha = 255) with the same blue color (#1a73e8).

### After Fix
```
32x32.png:      PNG image data, 32 x 32, 8-bit/color RGBA, non-interlaced
128x128.png:    PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced
128x128@2x.png: PNG image data, 256 x 256, 8-bit/color RGBA, non-interlaced
```

## Technical Details

The PNG header color type byte changed from:
- **RGB**: `0x02` (3 channels: R, G, B)
- **RGBA**: `0x06` (4 channels: R, G, B, A)

This can be verified by checking byte 25 in the PNG file header:
- Before: `08 02` (8-bit RGB)
- After: `08 06` (8-bit RGBA)

## Files Modified

- `src-tauri/icons/32x32.png` - Converted to RGBA
- `src-tauri/icons/128x128.png` - Converted to RGBA
- `src-tauri/icons/128x128@2x.png` - Converted to RGBA

## Verification

Build should now succeed without the "icon is not RGBA" error.
