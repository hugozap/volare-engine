# JSONL Diagram Specification
v1.0

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
A standalone rectangle shape.

**Optional Attributes:**
- `width` (number or string) - Width (default: 100)
- `height` (number or string) - Height (default: 100)
- `background`, `background_color`, or `fill` (string) - Fill color (default: "white")
- `border_color` or `stroke_color` (string) - Border color (default: "black")
- `border_width` or `stroke_width` (number) - Border thickness (default: 1)
- `border_radius` (number) - Corner radius (default: 0)

**Example:**
```json
{"id":"rect1","type":"rect","width":150,"height":100,"background":"red"}
```

### Vertical Stack (`"type": "vstack"`)
Arranges children vertically.

**Required Attributes:**
- `children` (array) - Array of child element IDs

**Optional Attributes:**
- `h_align` or `horizontal_alignment` (string) - Horizontal alignment: "left", "center", "right" (default: "center")

**Example:**
```json
{"id":"stack","type":"vstack","children":["item1","item2","item3"],"h_align":"left"}
```

### Horizontal Stack (`"type": "hstack"`)
Arranges children horizontally.

**Required Attributes:**
- `children` (array) - Array of child element IDs

**Optional Attributes:**
- `v_align` or `vertical_alignment` (string) - Vertical alignment: "top", "center", "bottom" (default: "center")

**Example:**
```json
{"id":"row","type":"hstack","children":["col1","col2","col3"],"v_align":"top"}
```

### Group (`"type": "group"`)
Groups elements together without layout constraints.

**Required Attributes:**
- `children` (array) - Array of child element IDs

**Example:**
```json
{"id":"group1","type":"group","children":["elem1","elem2"]}
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
- `cx` or `center_x` (number) - Center X coordinate
- `cy` or `center_y` (number) - Center Y coordinate
- `rx` or `radius_x` (number) - Horizontal radius
- `ry` or `radius_y` (number) - Vertical radius

**Optional Attributes:**
- `fill`, `fill_color`, or `background` (string) - Fill color (default: "white")
- `stroke`, `stroke_color`, or `border_color` (string) - Border color (default: "black")
- `stroke_width` or `border_width` (number) - Border thickness (default: 1)

**Example:**
```json
{"id":"circle","type":"ellipse","cx":50,"cy":50,"rx":25,"ry":25,"fill":"yellow"}
```

### Arc (`"type": "arc"`)
Draws an arc segment.

**Required Attributes:**
- `cx` or `center_x` (number) - Center X coordinate
- `cy` or `center_y` (number) - Center Y coordinate
- `radius` or `r` (number) - Arc radius
- `start_angle` or `start` (number) - Start angle in degrees
- `end_angle` or `end` (number) - End angle in degrees

**Optional Attributes:**
- `fill` or `fill_color` (string) - Fill color (default: "none")
- `stroke` or `stroke_color` (string) - Stroke color (default: "black")
- `stroke_width` (number) - Stroke thickness (default: 1)
- `filled` (boolean) - Whether to fill the arc sector (default: false)

**Example:**
```json
{"id":"arc1","type":"arc","cx":100,"cy":100,"radius":50,"start_angle":0,"end_angle":90,"stroke_color":"red","filled":true}
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

## Container Types

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

## Size Behaviors

For `width` and `height` attributes, you can use:

- **Number** - Fixed size in pixels: `"width": 200`
- **"content"** or **"auto"** - Size based on content: `"width": "content"`
- **"grow"** - Take all available space: `"width": "grow"`

## Color Values

Colors can be specified as:
- Named colors: `"red"`, `"blue"`, `"lightgray"`
- Hex colors: `"#FF0000"`, `"#0066CC"`
- RGB colors: `"rgb(255,0,0)"`

## Custom Components

The system supports custom components registered by the application. These components can accept any attributes defined by their implementation.

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