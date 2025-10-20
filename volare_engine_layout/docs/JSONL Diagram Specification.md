# JSONL Diagram Specification (Complete Reference)
v1.2 - Complete specification based on parser implementation

This specification describes the JSON Lines format for creating documents using the Volare Layout Engine. Each line in a JSONL file represents a single entity with its properties.

## Basic Structure

**CRITICAL: The first line of every JSONL document MUST be the root element.**

### Format Rules:
- **First line is always root** - The very first object in the file must be your root container (typically `id:"root"`) with a `children` array referencing other elements
- **One JSON object per line** - no formatted/pretty JSON  
- **No blank lines** between objects
- **Children can be referenced before being defined** - forward references are supported
- **All IDs must be unique** within the document

### Example:
```jsonl
{"id":"root","type":"vstack","children":["header","body"]}
{"id":"header","type":"text","content":"Title"}
{"id":"body","type":"text","content":"Content"}
```
❌ **WRONG - root is not first:**
```jsonl
{"id":"header","type":"text","content":"Title"}
{"id":"root","type":"vstack","children":["header"]}
```

---

## Core Entity Types

### Text (`"type": "text"`)
Renders text content with styling options.

**Required Attributes:**
- `content` or `text` (string) - The text to display

**Optional Attributes:**
- `font_size` (number) - Font size in pixels (default: 12)
- `color` or `text_color` (string) - Text color (default: "black")
- `font_family` (string) - Font family name (default: "Arial")
- `line_width` (number) - Maximum characters per line for wrapping (default: 200)
- `line_spacing` (number) - Space between lines (default: 0)
- `x` (number) - X position in free_container
- `y` (number) - Y position in free_container

**Example:**
```json
{"id":"title","type":"text","content":"Hello World","font_size":24,"color":"blue"}
{"id":"positioned_text","type":"text","content":"Positioned","x":50,"y":100}
```

---

### Box (`"type": "box"`)
Wraps a single child element with padding, background, and border.

**Required Attributes:**
- `children` (array with exactly one element) - Child element ID

**Optional Attributes:**
- `padding` (number) - Inner padding (default: 0)
- `background`, `background_color`, or `fill` (string) - Background color (default: "white")
- `border_color` or `stroke_color` (string) - Border color (default: "black")
- `border_width` or `stroke_width` (number) - Border thickness (default: 1)
- `border_radius` (number) - Corner radius (default: 0)
- `width` (number or "content"/"grow") - Width behavior
- `height` (number or "content"/"grow") - Height behavior
- `x` (number) - X position in free_container
- `y` (number) - Y position in free_container

**Example:**
```json
{"id":"container","type":"box","padding":10,"background":"lightblue","border_radius":5,"children":["text1"]}
```

**Note:** Children array must contain exactly one element ID.

---

### Rectangle (`"type": "rect"`)
Draws a rectangle shape.

**Required Attributes:**
- `width` (number or "content"/"grow") - Rectangle width
- `height` (number or "content"/"grow") - Rectangle height

**Optional Attributes:**
- `background`, `background_color`, or `fill` (string) - Fill color (default: "white")
- `border_color`, `stroke_color`, or `stroke` (string) - Border color (default: "black")
- `border_width` or `stroke_width` (number) - Border thickness (default: 1)
- `border_radius` (number) - Corner radius (default: 0)
- `x` (number) - X position in free_container
- `y` (number) - Y position in free_container

**Example:**
```json
{"id":"rect1","type":"rect","width":100,"height":50,"fill":"red","border_radius":5}
{"id":"rect2","type":"rect","width":80,"height":60,"fill":"blue","x":150,"y":100}
```

---

### Ellipse (`"type": "ellipse"`)
Draws an ellipse or circle.

**Required Attributes:**
- `rx` or `radius_x` (number) - Horizontal radius
- `ry` or `radius_y` (number) - Vertical radius

**Optional Attributes:**
- `fill`, `fill_color`, or `background` (string) - Fill color (default: "white")
- `stroke`, `stroke_color`, or `border_color` (string) - Stroke color (default: "black")
- `stroke_width` or `border_width` (number) - Stroke width (default: 1)
- `x` (number) - X position in free_container
- `y` (number) - Y position in free_container

**Example:**
```json
{"id":"circle","type":"ellipse","rx":25,"ry":25,"fill":"yellow","stroke":"orange","stroke_width":2}
{"id":"ellipse","type":"ellipse","rx":40,"ry":20,"fill":"lightblue","x":100,"y":50}
```

**Important Notes:**
- Element size is `2*rx` by `2*ry`
- `cx` and `cy` attributes are parsed but IGNORED - use `x`, `y` for positioning
- Ellipse is centered within its bounding box

---

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
- `x` (number) - X position of line's bounding box in free_container
- `y` (number) - Y position of line's bounding box in free_container

**Example:**
```json
{"id":"line1","type":"line","start_x":0,"start_y":0,"end_x":100,"end_y":50,"stroke_color":"blue","stroke_width":2}
```

---

### Arc (`"type": "arc"`)
Draws an arc segment.

**Required Attributes:**
- `radius` or `r` (number) - Arc radius (default: 50)
- `start_angle` or `start` (number) - Start angle in degrees (default: 0)
- `end_angle` or `end` (number) - End angle in degrees (default: 90)

**Optional Attributes:**
- `fill` or `fill_color` (string) - Fill color (default: "none")
- `stroke` or `stroke_color` (string) - Stroke color (default: "black")
- `stroke_width` (number) - Stroke thickness (default: 1)
- `filled` (boolean) - Whether to fill the arc sector (default: false)
- `x` (number) - X position in free_container
- `y` (number) - Y position in free_container

**Example:**
```json
{"id":"quarter","type":"arc","radius":40,"start_angle":0,"end_angle":90,"stroke_color":"red","stroke_width":2}
{"id":"semicircle","type":"arc","radius":30,"start_angle":0,"end_angle":180,"filled":true,"fill_color":"blue"}
{"id":"circle","type":"arc","radius":25,"start_angle":0,"end_angle":360,"filled":true,"fill_color":"green"}
```

**Notes:**
- Arc size is always `diameter = radius * 2`
- Arc is centered within its bounding box
- 360° arcs render as full circles
- Filled arcs create pie-slice shapes

---

### Semicircle (`"type": "semicircle"`)
Convenience type for 180° arcs.

**Required Attributes:**
- `radius` or `r` (number) - Semicircle radius (default: 50)

**Optional Attributes:**
- `facing_up` or `up` (boolean) - True for top (180°-360°), false for bottom (0°-180°) (default: true)
- `fill` or `fill_color` (string) - Fill color (default: "none")
- `stroke` or `stroke_color` (string) - Stroke color (default: "black")
- `stroke_width` (number) - Stroke thickness (default: 1)
- `filled` (boolean) - Whether to fill (default: false)
- `x` (number) - X position in free_container
- `y` (number) - Y position in free_container

**Example:**
```json
{"id":"semi1","type":"semicircle","radius":40,"facing_up":false,"filled":true,"fill_color":"green"}
```

---

### Quarter Circle (`"type": "quarter_circle"`)
Convenience type for 90° arcs.

**Required Attributes:**
- `radius` or `r` (number) - Radius (default: 50)
- `quadrant` (number) - Quadrant: 1, 2, 3, or 4 (default: 1)

**Optional Attributes:**
- `fill` or `fill_color` (string) - Fill color (default: "none")
- `stroke` or `stroke_color` (string) - Stroke color (default: "black")
- `stroke_width` (number) - Stroke thickness (default: 1)
- `filled` (boolean) - Whether to fill (default: false)
- `x` (number) - X position in free_container
- `y` (number) - Y position in free_container

**Example:**
```json
{"id":"quarter1","type":"quarter_circle","radius":30,"quadrant":1,"filled":true,"fill_color":"orange"}
```

---

### Polyline (`"type": "polyline"`)
Draws connected line segments through multiple points.

**Required Attributes:**
- `points` (array of [x,y] arrays) - Coordinate pairs

**Optional Attributes:**
- `stroke_color` or `color` (string) - Line color (default: "black")
- `stroke_width` (number) - Line thickness (default: 1)
- `x` (number) - X position of polyline's bounding box in free_container
- `y` (number) - Y position of polyline's bounding box in free_container

**Example:**
```json
{"id":"poly1","type":"polyline","points":[[0,0],[50,25],[100,0],[150,50]],"stroke_color":"purple","stroke_width":2}
```

---

### Image (`"type": "image"`)
Displays an image from URL or file path.

**Required Attributes (one of):**
- `src` (string) - Image URL or base64 data
- `file_path` (string) - Path to image file

**Required Size:**
- `width` (number or "content"/"grow") - Image width
- `height` (number or "content"/"grow") - Image height

**Optional Attributes:**
- `x` (number) - X position in free_container
- `y` (number) - Y position in free_container

**Example:**
```json
{"id":"img1","type":"image","src":"https://example.com/logo.png","width":200,"height":150}
{"id":"img2","type":"image","file_path":"assets/photo.jpg","width":300,"height":200,"x":50,"y":50}
```

---

### Connector (`"type": "connector"`)
Automatically draws a line connecting two elements from their centers.

**Required Attributes:**
- `source`, `source_id`, or `from` (string) - Source element ID
- `target`, `target_id`, or `to` (string) - Target element ID

**Optional Attributes:**
- `stroke_color`, `color`, or `stroke` (string) - Line color (default: "black")
- `stroke_width` (number) - Line width (default: 1)
- `connector_type` (string) - "straight", "curved", or "orthogonal" (default: "straight")
- `curve_offset` or `curve_amount` (number) - Curve offset for "curved" type



**Optional Port Attributes:**
- `source_port` (string) - Port on source element: "center", "top", "bottom", "left", "right", "top_left", "top_right", "bottom_left", "bottom_right" (default: "center")
- `target_port` (string) - Port on target element (same options as source_port)

**Example:**
```json
{"id":"conn1","type":"connector","source":"box1","source_port":"right","target":"box2","target_port":"left","stroke_color":"red","stroke_width":2}
{"id":"conn2","type":"connector","source":"header","source_port":"bottom","target":"body","target_port":"top","stroke_color":"blue","stroke_width":2}
```

**Example:**
```json
{"id":"conn1","type":"connector","source":"box1","target":"box2","stroke_color":"red","stroke_width":2}
{"id":"conn2","type":"connector","from":"elem1","to":"elem2","connector_type":"curved","curve_offset":20}
```
**Optional Arrow Attributes:**
- `arrow_start` or `arrow_begin` (boolean) - Add arrowhead at source end (default: false)
- `arrow_end` (boolean) - Add arrowhead at target end (default: false)
- `arrow_size` (number) - Size of arrowheads (default: 8.0)

**Example:**
```json
{"id":"conn1","type":"connector","source":"box1","target":"box2","arrow_end":true,"stroke_color":"blue","stroke_width":2}
{"id":"conn2","type":"connector","source":"box2","target":"box3","arrow_start":true,"arrow_end":true,"stroke_color":"red","stroke_width":2}
{"id":"conn3","type":"connector","source":"box1","source_port":"right","target":"box2","target_port":"left","arrow_end":true,"arrow_size":12,"stroke_color":"green","stroke_width":3}
```

**Notes:**
- Connectors calculate positions automatically based on source/target centers
- Must be in a container that can access both source and target elements
- Connectors auto-promote in tree structure to ensure element access

---

### Spacer (`"type": "spacer"`)
Creates spacing between elements.

**Optional Attributes:**
- `width` (number) - Width (default: 1)
- `height` (number) - Height (default: 20)
- `direction` (string) - "horizontal", "vertical", or "both" (default: "vertical")

**Example:**
```json
{"id":"spacer1","type":"spacer","height":30}
{"id":"spacer2","type":"spacer","width":50,"direction":"horizontal"}
```

---

## Layout Containers

### Vertical Stack (`"type": "vstack"`)
Stacks children vertically.

**Required Attributes:**
- `children` (array) - Child element IDs

**Optional Attributes:**
- `h_align` or `horizontal_alignment` (string) - "left", "center", "right" (default: "center")
- `spacing` (number) - Vertical spacing between children (default: 0)
- `x` (number) - X position in free_container
- `y` (number) - Y position in free_container

**Example:**
```json
{"id":"stack1","type":"vstack","children":["elem1","elem2","elem3"],"h_align":"left","spacing":10}
```

---

### Horizontal Stack (`"type": "hstack"`)
Stacks children horizontally.

**Required Attributes:**
- `children` (array) - Child element IDs

**Optional Attributes:**
- `v_align` or `vertical_alignment` (string) - "top", "center", "bottom" (default: "center")
- `spacing` (number) - Horizontal spacing between children (default: 0)
- `x` (number) - X position in free_container
- `y` (number) - Y position in free_container

**Example:**
```json
{"id":"stack2","type":"hstack","children":["elem1","elem2"],"v_align":"top","spacing":15}
```

---

### Group (`"type": "group"`)
Groups elements without layout constraints.

**Required Attributes:**
- `children` (array) - Child element IDs

**Example:**
```json
{"id":"group1","type":"group","children":["elem1","elem2","elem3"]}
```

**Note:** Group imposes no layout - children render in natural positions.

---

### Table (`"type": "table"`)
Arranges children in grid layout.

**Required Attributes:**
- `children` (array) - Cell element IDs (filled row by row)
- `cols` or `columns` (number) - Number of columns

**Optional Attributes:**
- `header_fill_color` or `header_background` (string) - First row background (default: "lightgray")
- `fill_color` or `background` (string) - Table background (default: "white")
- `border_color` (string) - Border color (default: "black")
- `border_width` (number) - Border thickness (default: 1)
- `cell_padding` or `padding` (number) - Cell padding (default: 20)

**Example:**
```json
{"id":"table1","type":"table","children":["h1","h2","c1","c2","c3","c4"],"cols":2,"cell_padding":15}
```

---

### Free Container (`"type": "free_container"`)
Allows absolute positioning of children using x,y coordinates.

**Required Attributes:**
- `children` (array) - Child element IDs

**Optional Attributes:**
- `width` (number) - Container width
- `height` (number) - Container height
- `background` or `background_color` (string) - Background color
- `border_color` (string) - Border color
- `border_width` (number) - Border thickness
- `x` (number) - X position in parent free_container
- `y` (number) - Y position in parent free_container

**Example:**
```json
{"id":"container","type":"free_container","width":400,"height":300,"children":["item1","item2"]}
{"id":"item1","type":"text","content":"Text","x":50,"y":100}
{"id":"item2","type":"rect","width":60,"height":40,"fill":"blue","x":200,"y":150}
```

**Critical:** Each child MUST have `x` and `y` attributes for positioning.

---

### Constraint Container (`"type": "constraint_container"`)
Uses constraint-based layout for positioning.

**Required Attributes:**
- `children` (array) - Child element IDs

**Optional Attributes:**
- `constraints` (array) - Constraint objects (see Constraints section)

**Example:**
```json
{"id":"layout","type":"constraint_container","children":["box1","box2"],"constraints":[{"type":"align_left","entities":["box1","box2"]},{"type":"below","entities":["box2","box1"]}]}
```

---

## Transform Attributes

All elements support transform attributes:

- `rotation` (number) - Rotation in degrees (around element center)
- `scale_x` (number) - Horizontal scale factor
- `scale_y` (number) - Vertical scale factor
- `transform` (string) - CSS-style transform (experimental, limited support)

**Example:**
```json
{"id":"rotated","type":"rect","width":50,"height":30,"fill":"blue","rotation":45}
{"id":"scaled","type":"text","content":"Big","scale_x":2.0,"scale_y":1.5}
```

---

## Size Behaviors

For `width` and `height` attributes:

- **Number** - Fixed size in pixels: `100`
- **"content"** - Size based on content (default for most elements)
- **"grow"** - Expand to fill available space

**Example:**
```json
{"id":"fixed","type":"rect","width":100,"height":50}
{"id":"content_sized","type":"box","width":"content","height":"content","children":["text1"]}
{"id":"growing","type":"rect","width":"grow","height":50}
```

---

## Color Values

Supported color formats:
- Named: `"red"`, `"blue"`, `"lightgray"`
- Hex: `"#FF0000"`, `"#0066CC"`, `"#f0f0f0"`
- RGB: `"rgb(255,0,0)"`

---

## Attribute Aliases

The parser accepts multiple names for the same attribute:

| Concept | Aliases |
|---------|---------|
| Background color | `background`, `background_color`, `fill` |
| Border/stroke color | `border_color`, `stroke_color`, `stroke` |
| Border/stroke width | `border_width`, `stroke_width` |
| Text content | `content`, `text` |
| Text color | `color`, `text_color` |
| Source | `source`, `source_id`, `from` |
| Target | `target`, `target_id`, `to` |
| Radius X | `rx`, `radius_x` |
| Radius Y | `ry`, `radius_y` |
| Radius | `radius`, `r` |
| Columns | `cols`, `columns` |
| Cell padding | `padding`, `cell_padding` |
| H-alignment | `h_align`, `horizontal_alignment` |
| V-alignment | `v_align`, `vertical_alignment` |
| Line start X | `start_x`, `x1` |
| Line start Y | `start_y`, `y1` |
| Line end X | `end_x`, `x2` |
| Line end Y | `end_y`, `y2` |
| Arc start | `start_angle`, `start` |
| Arc end | `end_angle`, `end` |

---

## Constraint Types

Constraints are ONLY valid within `constraint_container`.

### Alignment Constraints
Align multiple entities. First entity is reference.

```json
{"type":"align_left","entities":["rect1","rect2","rect3"]}
{"type":"align_right","entities":["rect1","rect2"]}
{"type":"align_top","entities":["rect1","rect2","rect3"]}
{"type":"align_bottom","entities":["rect1","rect2"]}
{"type":"align_center_horizontal","entities":["rect1","rect2"]}
{"type":"align_center_vertical","entities":["rect1","rect2","rect3"]}
```

### Directional Positioning
Position the first entity relative to the second entity (reference).
```json
{"type":"right_of","entities":["elem_A","elem_B"]}  // A is to the right of B
{"type":"left_of","entities":["elem_A","elem_B"]}   // A is to the left of B
{"type":"above","entities":["elem_A","elem_B"]}     // A is above B
{"type":"below","entities":["elem_A","elem_B"]}     // A is below B
```

**Syntax:** `{"type":"<direction>","entities":[<positioned>,<reference>]}`
- First element is the one being positioned
- Second element is the reference/anchor element

**Examples:**
```json
// Place header above content
{"type":"above","entities":["header","content"]}

// Place footer below content  
{"type":"below","entities":["footer","content"]}

// Place sidebar to the left of main area
{"type":"left_of","entities":["sidebar","main"]}
```
### Spacing Constraints

```json
{"type":"horizontal_spacing","entities":["rect1","rect2"],"spacing":20.0}
{"type":"vertical_spacing","entities":["rect1","rect2"],"spacing":15.0}
{"type":"fixed_distance","entities":["rect1","rect2"],"distance":100.0}
```

### Layout Stacking

```json
{"type":"stack_horizontal","entities":["rect1","rect2","rect3"],"spacing":10.0}
{"type":"stack_vertical","entities":["rect1","rect2","rect3"],"spacing":8.0}
```

### Size Constraints

```json
{"type":"same_width","entities":["rect1","rect2","rect3"]}
{"type":"same_height","entities":["rect1","rect2"]}
{"type":"same_size","entities":["rect1","rect2"]}
{"type":"at_least_same_height","entities":["rect1","rect2"]}
{"type":"proportional_width","entities":["rect1","rect2"],"ratio":1.5}
{"type":"proportional_height","entities":["rect1","rect2"],"ratio":0.8}
{"type":"min_height","entity":"rect1","height":50.0}
```

### Advanced Constraints

```json
{"type":"aspect_ratio","entity":"rect1","ratio":1.618}
{"type":"distribute_horizontally","entities":["rect1","rect2","rect3"]}
{"type":"distribute_vertically","entities":["rect1","rect2","rect3"]}
```

---

## Complete Examples

### Basic Document
```jsonl
{"id":"root","type":"vstack","children":["header","content","footer"],"h_align":"center","spacing":10}
{"id":"header","type":"box","padding":15,"background":"#2c3e50","children":["title"]}
{"id":"title","type":"text","content":"My Document","font_size":24,"color":"white"}
{"id":"content","type":"hstack","children":["sidebar","main"],"v_align":"top","spacing":20}
{"id":"sidebar","type":"vstack","children":["nav1","nav2","nav3"],"h_align":"left","spacing":5}
{"id":"nav1","type":"text","content":"Home","color":"blue"}
{"id":"nav2","type":"text","content":"About","color":"blue"}
{"id":"nav3","type":"text","content":"Contact","color":"blue"}
{"id":"main","type":"box","padding":20,"background":"white","children":["article"]}
{"id":"article","type":"text","content":"Main content area.","line_width":300}
{"id":"footer","type":"text","content":"© 2024","font_size":10,"color":"gray"}
```

### Free Container with Connectors
```jsonl
{"id":"root","type":"free_container","width":500,"height":300,"children":["box1","box2","box3","conn1","conn2"]}
{"id":"box1","type":"rect","x":50,"y":120,"width":80,"height":60,"fill":"lightblue","stroke":"blue","stroke_width":2}
{"id":"box2","type":"rect","x":220,"y":120,"width":80,"height":60,"fill":"lightgreen","stroke":"green","stroke_width":2}
{"id":"box3","type":"rect","x":390,"y":120,"width":80,"height":60,"fill":"lightyellow","stroke":"orange","stroke_width":2}
{"id":"conn1","type":"connector","source":"box1","target":"box2","stroke_color":"blue","stroke_width":2}
{"id":"conn2","type":"connector","source":"box2","target":"box3","stroke_color":"green","stroke_width":2}
```

### Constraint Layout
```jsonl
{"id":"rect1","type":"rect","width":100,"height":60,"fill":"blue"}
{"id":"rect2","type":"rect","width":100,"height":60,"fill":"red"}
{"id":"rect3","type":"rect","width":100,"height":60,"fill":"green"}
{"id":"root","type":"constraint_container","children":["rect1","rect2","rect3"],"constraints":[{"type":"stack_horizontal","entities":["rect1","rect2","rect3"],"spacing":20.0},{"type":"align_center_vertical","entities":["rect1","rect2","rect3"]},{"type":"same_height","entities":["rect1","rect2","rect3"]}]}
```

---

## Custom Components

Custom components are registered via `builder.register_custom_component()` and can accept any attributes their implementation defines.

**Example:**
```json
{"id":"badge1","type":"badge","text":"NEW","background":"red"}
```

---

## Implementation References

- **Parser:** `volare_engine_layout/src/parser.rs`
- **Layout Engine:** `volare_engine_layout/src/layout.rs`
- **Components:** `volare_engine_layout/src/components.rs`
- **Constraints:** `volare_engine_layout/src/constraints/mod.rs`

---

**Version 1.2** - Complete specification based on parser implementation