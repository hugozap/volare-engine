# The diagram Tree.

Diagrams are declared as a tree of nodes.
Each node must contain one of the supported diagram components:

- Box
- Group
- Text
- VerticalStack
- etc.

Each diagram consists in the logic necessary to create the tree, without regard for the element size or position, that's calculated by the engine.


The layout components

Layout components that are containers, expect their children to already exist. So we need an upper layer that creates an abstraction so the user is not concerned with internal details.

Ideally the user defines something like:

VerticalStack({props}, [
    Box ({props}, elem),
    HorizontalStack({props}, [
        elem1,
        elem2,
        elem3
    ]),
    Table({props}, [
        [elem1,elem2,elem3],
        [elem4,elem5,elem6]
    ])
])

What we are building is something similar to a virtual representation.
Like react virtual dom.


Each function should return a DiagramNode


Another option is to have vectors for each type of node in the parent element,
and just store the index in DiagramNode

```
struct DiagramNode {
    entity_type: EntityType,
    ix: usize,
    children: Vec<(ix, EntityType)>
}
```

This way we would have a way to retrieve the specific element from a different array.

Where would those arrays live?

Some kind of DiagramBuilder object

## Builder pattern

```rust
let builder = DiagramBuilder::new();

let root = builder.newVerticalStack()
root.add(builder.newBox(...))
.add(builder.newHorizontalStack(...).
    add(builder.newBox({}, builder.Text("Hey"))))

```

Having a builder api with methods that make it easy to create a diagram
by adding nodes is useful to create the API

Then, we can pass the DiagramNode object to the render api.

What happens inside newBox method inside the builder?

- The builder populates the BoxShape component and adds it to the boxes list
- Returns the position of the item in the array **and the entity type**
- (ix, EntityType)

Inside `add`:

Each children is a pair of (ix, EntityType).

Now we have a space where the properties are stored, and each node store indexes and an identifier
to have a way to know what array to look for.

Each index is valid only for the lifetime of the builder object.

## Stage 2

We already have a diagram representation, but no entities have been created.
The properties for each component will be required by the renderer layer (e.g color).
So they should be passed to the render layer somehow.

The renderer can receive an instance of the builder to have access to the internal details for each element?

The properties do affect layout and are used by rendering.



