
These components are higher level elements used to present information with good typography and spacing settings.

## document

the root document container, 

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

The section provides a multi column layout, the columns attribute contains the array of ids of the elements that contain the column content.

columns can also be sections for composing more complex layouts

## properties

A component used to present information in a table based with two columns, first column is the attribute name, and second column is the attribute value

```
{"id":"properties-car", "type":"properties", "items"=[{"name":"mileage", "value":"15000}]}
```

## document.text

A text paragraph with good default typography settings with optional title

```
{"id":"explanation", "type":"document.text", "variant":"default|large|small|subtle|emphasized "text":"the content" "title": "paragraph title"}
```

default: Standard content blocks
large: Hero sections or main headings
small: Compact spaces or secondary information
subtle: Labels or supporting content
emphasized: Important callouts or quotes

typeface: Helvetica

## bullet-list

A bullet point list of elements with good typography and spacing settings

```
{"id":"item-list", "type":"bullet-list", "items":["first","second","third"]}
```

## numbered-list

A numbered list of elements with good typography and spacing settings

```
{"id":"item-list", "type":"numbered-list", "items":["first","second","third"]}
```

## card

attributes

- meta : information that describes or categorizes the main content
- title : string
- content: string (if the content is text)
- content_id: (if other element should be instantiated in the content area)


## quote-block

Left border emphasis for important information

## code-block

Monospace content with background differentiation

