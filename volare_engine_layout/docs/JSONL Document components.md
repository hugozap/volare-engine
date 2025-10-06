# JSONL Document components Specification (high level document components)

These components are higher level elements used to present information with good typography and spacing settings. 

## document

the root document container
**Always use the `document` element as the root**

attributes
- header_id (optional)
- content_id (optional)
- footer_id (optional)

## document.section

```
{"id":"section-example", "type":"document.section", "title":"the section title",  "meta":"Design Theory - 2024", "columns":["col1","col2","col3"]}
```

attributes
- title (string - optional)
- columns: (array of ids)
- meta: (optional) complementary text

The section provides a multi column layout, the columns attribute contains the array of ids of the elements that contain the column content.

columns can also be sections for composing more complex layouts

## document.properties

A component used to present information in a table based with two columns, first column is the attribute name, and second column is the attribute value

```
{"id":"properties-car", "type":"document.properties", "meta":"Car Properties", "properties"=[["name", "mustang"], ["value":"3000"]]}
```

Attributes:
- properties: A list of two value lists (first is name of the property and second is the value, both strings)
- meta: (optional) a short text that is display on top of the element to give some context to the reader.

## document.text

A text paragraph with good default typography settings with optional title

```
{"id":"explanation", "type":"document.text", "variant":"default|large|small|subtle|emphasized "text":"the content" "title": "paragraph title", "width":"sm|md|lg|xl|full|<number>"}
```

Variants: 

default: Standard content blocks
large: Hero sections or main headings
small: Compact spaces or secondary information
subtle: Labels or supporting content
emphasized: Important callouts or quotes

width (optional)

Any of the following strings or 
"sm" : Small width (approx 480px)
"md" : Medium width (approx 640px)
"lg" : Large (840px)
"xl" : X-Large (1024)
"full": 1200

width can also be a number e.g "300"


## document.bullet_list

A bullet point list of elements with good typography and spacing settings

```
{"id":"item-list", "type":"document.bullet_list", "meta":"A useful list" "items":["first","second","third"]}
```


## numbered-list (NOT IMPLEMENTED YET!)

A numbered list of elements with good typography and spacing settings

```
{"id":"item-list", "type":"numbered-list", "items":["first","second","third"]}
```

## card (NOT IMPLEMENTED YET!)

attributes

- meta : information that describes or categorizes the main content
- title : string
- content: string (if the content is text)
- content_id: (if other element should be instantiated in the content area)


## quote-block (NOT IMPLEMENTED YET)

Left border emphasis for important information

## code-block (NOT IMPLEMENTED YET)

Monospace content with background differentiation

