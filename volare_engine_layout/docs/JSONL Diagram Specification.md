# JSONL Diagram Specification
v1.1 - 

This specification describes the JSON Lines format for creating diagrams using the Volare Layout Engine. Each line in a JSONL file represents a single entity with its properties.

## Basic Structure

Each entity follows this format:
```json
{"id": "unique_id", "type": "entity_type", "attribute1": "value1", "attribute2": "value2"}
```

## Core Entity Types

### Text (`"type": "text"`)
Renders text content with styling options.

**Required Attributes:**
- `content` or `text` - The text to display

**Optional Attributes:**
- `font_size` (number) - Font size in pixels (default: 12)
- `color` or `text_color` (string) - Text color (default: "black")
- `font_family` (string) - Font family name (default: "Arial")
- `line_width` (number) - Maximum characters per line for wrapping (default: 200)
- `line_spacing` (number) - Space between lines (default: 0)

**Example:**
```json
{"id":"title","type":"text","content":"Hello World","font_size":24,"color":"blue"}
```

### Box (`"type": "box"`)
Wraps another element with padding, background, and border.

**Required Attributes:**
- `children` (array) - Array with exactly one child element ID

**Optional Attributes:**
- `padding` (number) - Inner padding (default: 0)
- `background`, `background_color`, or `fill` (string) - Background color (default: "white")
- `border_color` or `stroke_color` (string) - Border color (default: "black")
- `border_width` or `stroke_width` (number) - Border thickness (default: 1)
- `border_radius` (number) - Corner radius for rounded corners (default: 0)
- `width` (number or string) - Width behavior: number for fixed, "content" for auto, "grow" for fill
- `height` (number or string) - Height behavior: number for fixed, "content" for auto, "grow" for fill

**Example:**
```json
{"id":"container","type":"box","padding":10,"background":"lightblue","children":["text1"]}
```

### Rectangle (`"type": "rect"`)
Draws a rectangle shape.

**Required Attributes:**
- `width` (number) - Rectangle width
- `height` (number) - Rectangle height

**Optional Attributes:**
- `background`, `background_color`, or `fill` (string) - Fill color (default: "white")
- `border_color` or `stroke_color` (string) - Border color (default: "black")
- `border_width` or `stroke_width` (number) - Border thickness (default: 1)
- `border_radius` (number) - Corner radius for rounded corners (default: 0)

**Example:**
```json
{"id":"rect1","type":"rect","width":100,"height":50,"background":"red","border_radius":5}
```

## Layout Containers

### Vertical Stack (`"type": "vstack"`)
Stacks children vertically.

**Required Attributes:**
- `children` (array) - Array of child element IDs

**Optional Attributes:**
- `h_align` or `horizontal_alignment` (string) - Horizontal alignment: "left", "center", "right" (default: "center")

**Example:**
```json
{"id":"stack1","type":"vstack","children":["elem1","elem2"],"h_align":"left"}
```

### Horizontal Stack (`"type": "hstack"`)
Stacks children horizontally.

**Required Attributes:**
- `children` (array) - Array of child element IDs

**Optional Attributes:**
- `v_align` or `vertical_alignment` (string) - Vertical alignment: "top", "center", "bottom" (default: "center")

**Example:**
```json
{"id":"stack2","type":"hstack","children":["elem1","elem2"],"v_align":"top"}
```

### Group (`"type": "group"`)
Groups elements without layout constraints.

**Required Attributes:**
- `children` (array) - Array of child element IDs

**Example:**
```json
{"id":"group1","type":"group","children":["elem1","elem2"]}
```

### Table (`"type": "table"`)
Arranges children in a grid layout.

**Required Attributes:**
- `children` (array) - Array of cell element IDs
- `cols` or `columns` (number) - Number of columns

**Optional Attributes:**
- `header_fill_color` or `header_background` (string) - Header row background (default: "lightgray")
- `fill_color` or `background` (string) - Table background (default: "white")
- `border_color` (string) - Border color (default: "black")
- `border_width` (number) - Border thickness (default: 1)
- `cell_padding` or `padding` (number) - Cell padding (default: 20)

**Example:**
```json
{"id":"table1","type":"table","children":["cell1","cell2","cell3","cell4"],"cols":2,"cell_padding":15}
```

### Free Container (`"type": "free_container"`)
Allows absolute positioning of children.

**Required Attributes:**
- `children` (array) - Array of child element IDs

**Child Positioning:**
Each child element should have `x` and `y` attributes for positioning within the container.

**Optional Container Attributes:**
- `width` (number) - Container width
- `height` (number) - Container height
- `background` or `background_color` (string) - Background color
- `border_color` (string) - Border color
- `border_width` (number) - Border thickness

**Example:**
```json
{"id":"container","type":"free_container","width":400,"height":300,"children":["item1","item2"]}
{"id":"item1","type":"text","content":"Positioned Text","x":50,"y":100}
{"id":"item2","type":"rect","width":60,"height":40,"background":"blue","x":200,"y":150}
```

## Shape Types

### Line (`"type": "line"`)
Draws a straight line between two points.

**Required Attributes:**
- `start_x` or `x1` (number) - Starting X coordinate
- `start_y` or `y1` (number) - Starting Y coordinate
- `end_x` or `x2` (number) - Ending X coordinate
- `end_y` or `y2` (number) - Ending Y coordinate

**Optional Attributes:**
- `stroke_color` or `color` (string) - Line color (default: "black")
- `stroke_width` (number) - Line thickness (default: 1)

**Example:**
```json
{"id":"line1","type":"line","start_x":0,"start_y":0,"end_x":100,"end_y":50,"stroke_color":"blue"}
```

### Ellipse (`"type": "ellipse"`)
Draws an ellipse or circle.

**Required Attributes:**
- `rx` or `radius_x` (number) - Horizontal radius
- `ry` or `radius_y` (number) - Vertical radius

**Optional Attributes:**
- `fill`, `fill_color`, or `background` (string) - Fill color (default: "white")
- `stroke`, `stroke_color`, or `border_color` (string) - Border color (default: "black")
- `stroke_width` or `border_width` (number) - Border thickness (default: 1)

**Example:**
```json
{"id":"circle","type":"ellipse","rx":25,"ry":25,"fill":"yellow"}
```

### Arc (`"type": "arc"`)
Draws an arc segment. Arcs work like ellipses - they are positioned by the layout system and centered within their bounding box.

**Required Attributes:**
- `radius` or `r` (number) - Arc radius
- `start_angle` or `start` (number) - Start angle in degrees
- `end_angle` or `end` (number) - End angle in degrees

**Optional Attributes:**
- `fill` or `fill_color` (string) - Fill color (default: "none")
- `stroke` or `stroke_color` (string) - Stroke color (default: "black")
- `stroke_width` (number) - Stroke thickness (default: 1)
- `filled` (boolean) - Whether to fill the arc sector (default: false)

**Positioning:**
- Arcs are positioned using `x`, `y` attributes (for free containers) like other shapes
- The `cx`, `cy` attributes are ignored - positioning is handled by the layout system
- Arc size is always `diameter = radius * 2` regardless of arc sweep
- The arc is automatically centered within its bounding box

**Special Cases:**
- 360° arcs (full circles) render as proper circles
- Filled arcs create pie-slice shapes with lines to the center
- Unfilled arcs draw only the arc curve

**Examples:**
```json
{"id":"quarter","type":"arc","radius":40,"start_angle":0,"end_angle":90,"stroke_color":"red","x":50,"y":50}
{"id":"semicircle","type":"arc","radius":30,"start_angle":0,"end_angle":180,"filled":true,"fill_color":"blue","x":150,"y":50}
{"id":"full_circle","type":"arc","radius":25,"start_angle":0,"end_angle":360,"filled":true,"fill_color":"green","x":250,"y":50}
```

### Semicircle (`"type": "semicircle"`)
Draws a semicircle (180° arc).

**Required Attributes:**
- `cx` or `center_x` (number) - Center X coordinate
- `cy` or `center_y` (number) - Center Y coordinate
- `radius` or `r` (number) - Semicircle radius

**Optional Attributes:**
- `facing_up` or `up` (boolean) - True for top semicircle, false for bottom (default: true)
- `fill` or `fill_color` (string) - Fill color (default: "none")
- `stroke` or `stroke_color` (string) - Stroke color (default: "black")
- `stroke_width` (number) - Stroke thickness (default: 1)
- `filled` (boolean) - Whether to fill the semicircle (default: false)

**Example:**
```json
{"id":"semi1","type":"semicircle","cx":100,"cy":100,"radius":40,"facing_up":false,"fill":"green"}
```

### Quarter Circle (`"type": "quarter_circle"`)
Draws a quarter circle (90° arc).

**Required Attributes:**
- `cx` or `center_x` (number) - Center X coordinate
- `cy` or `center_y` (number) - Center Y coordinate
- `radius` or `r` (number) - Quarter circle radius
- `quadrant` (number) - Quadrant: 1=top-right, 2=top-left, 3=bottom-left, 4=bottom-right

**Optional Attributes:**
- `fill` or `fill_color` (string) - Fill color (default: "none")
- `stroke` or `stroke_color` (string) - Stroke color (default: "black")
- `stroke_width` (number) - Stroke thickness (default: 1)
- `filled` (boolean) - Whether to fill the quarter circle (default: false)

**Example:**
```json
{"id":"quarter1","type":"quarter_circle","cx":50,"cy":50,"radius":30,"quadrant":1,"fill":"orange"}
```


### Polyline (`"type": "polyline"`)
Draws connected line segments.

**Required Attributes:**
- `points` (array) - Array of [x, y] coordinate pairs

**Optional Attributes:**
- `stroke_color` or `color` (string) - Line color (default: "black")
- `stroke_width` (number) - Line thickness (default: 1)

**Example:**
```json
{"id":"poly1","type":"polyline","points":[[0,0],[50,25],[100,0],[150,50]],"stroke_color":"purple"}
```

## Media Types

### Image (`"type": "image"`)
Displays an image from file or base64 data.

**Required Attributes:**
- `src` (string) - Base64 image data, OR
- `file_path` (string) - Path to image file

**Optional Attributes:**
- `width` (number or string) - Image width
- `height` (number or string) - Image height

**Example:**
```json
{"id":"img1","type":"image","file_path":"assets/logo.png","width":200,"height":150}
```

## Transform and Positioning Attributes

All entities support the following positioning and transform attributes:

### Positioning (for Free Container children)
- `x` (number) - X position relative to container
- `y` (number) - Y position relative to container

### Transforms
- `rotation` or `rotate` (number) - Rotation angle in degrees
- `scale` (number or array) - Uniform scale (number) or [scaleX, scaleY] (array)
- `transform` (string) - CSS-style transform string (experimental)

**Transform Examples:**
```json
{"id":"rotated_rect","type":"rect","width":50,"height":30,"background":"blue","rotation":45}
{"id":"scaled_text","type":"text","content":"Big Text","scale":2.0}
{"id":"complex_shape","type":"rect","width":40,"height":40,"scale":[1.5,0.8],"rotation":30}
```

## Size Behaviors

For `width` and `height` attributes, you can use:

- **Number** - Fixed size in pixels: `"width": 200`
- **"content"** or **"auto"** - Size based on content: `"width": "content"`
- **"grow"** - Take all available space: `"width": "grow"` (TODO: Not currently supported)

**Note:** When using fixed width with text content, automatic text wrapping ensures optimal text layout within the specified constraints.

## Color Values

Colors can be specified as:
- Named colors: `"red"`, `"blue"`, `"lightgray"`
- Hex colors: `"#FF0000"`, `"#0066CC"`
- RGB colors: `"rgb(255,0,0)"`

## Custom Components

The system supports custom components registered by the application. These components can accept any attributes defined by their implementation. Examples from the codebase include:

- `badge` - Creates a styled badge with text
- `alert` - Creates alert boxes with different types
- `progress_bar` - Creates progress indicators
- `button` - Creates interactive buttons

Custom components are registered via `builder.register_custom_component()` and can have any attributes their implementation supports.

## Complete Example

```json
{"id":"root","type":"vstack","children":["header","content","footer"],"h_align":"center"}
{"id":"header","type":"box","padding":15,"background":"#f0f0f0","children":["title"]}
{"id":"title","type":"text","content":"My Document","font_size":24,"color":"darkblue"}
{"id":"content","type":"hstack","children":["sidebar","main"],"v_align":"top"}
{"id":"sidebar","type":"vstack","children":["nav1","nav2","nav3"],"h_align":"left"}
{"id":"nav1","type":"text","content":"Home","color":"blue"}
{"id":"nav2","type":"text","content":"About","color":"blue"}
{"id":"nav3","type":"text","content":"Contact","color":"blue"}
{"id":"main","type":"box","padding":20,"background":"white","children":["article"]}
{"id":"article","type":"text","content":"This is the main content area with longer text that will wrap to multiple lines.","line_width":300}
{"id":"footer","type":"text","content":"© 2024 My Company","font_size":10,"color":"gray"}
```

This creates a document layout with header, sidebar navigation, main content area, and footer.

## Implementation Notes

- The parser supports flexible attribute naming (e.g., `background`, `background_color`, and `fill` all work for background colors)
- Transform attributes are parsed and applied during entity building
- Container-relative positioning (`x`, `y`) is handled separately from transforms
- Size behaviors allow for responsive layouts with content-based or fixed sizing
- All supported entity types are handled in the `build_entity` match statement in parser.rs