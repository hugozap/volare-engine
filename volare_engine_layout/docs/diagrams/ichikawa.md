# Ishikawa Diagram (Fishbone Diagram) - JSONL Specification

## Overview
The Ishikawa diagram, also known as a fishbone diagram or cause-and-effect diagram, is used to visualize the potential causes of a problem. The diagram consists of a central "spine" leading to the problem statement, with category "branches" extending from the spine, each containing items that represent causes.

## Basic Structure

```jsonl
{"type":"ishikawa","id":"unique_id","problem":"Problem Statement","categories":[...]}
```

## Parameters

### Root Level Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `type` | string | Yes | Must be `"ishikawa"` |
| `id` | string | Yes | Unique identifier for the diagram (e.g., `"root"`) |
| `problem` | string | Yes | The main problem or effect being analyzed. Displayed in the "head" box on the right side of the diagram |
| `categories` | array | Yes | Array of category objects representing major cause categories |

### Category Object

Each category represents a major classification of causes (e.g., People, Process, Technology, Environment).

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `name` | string | Yes | Name of the category (e.g., `"People"`, `"Process"`) |
| `items` | array | Yes | Array of item objects representing causes within this category |

**Automatic Distribution**: Categories are automatically distributed between top and bottom branches:
- First half of categories → top branches
- Second half of categories → bottom branches
- Example: 4 categories = 2 top, 2 bottom

### Item Object

Items represent individual causes or factors. Items can have nested children for sub-causes.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `name` | string | Yes | Name/description of the cause (e.g., `"High turnover"`, `"Lack of training"`) |
| `children` | array | No | Array of nested item objects representing sub-causes. Can be nested multiple levels deep |

**Automatic Distribution**: Items within each category are automatically distributed:
- First half of items → left side of branch
- Second half of items → right side of branch
- Example: 5 items = 3 left, 2 right

## Complete Examples

### Example 1: Simple Ishikawa Diagram

```jsonl
{"type":"ishikawa","id":"root","problem":"Customer Complaints","categories":[{"name":"People","items":[{"name":"Insufficient training"},{"name":"High turnover"}]},{"name":"Process","items":[{"name":"No quality checks"},{"name":"Unclear procedures"}]}]}
```

**Result:**
- **Top branch**: People (Insufficient training on left, High turnover on right)
- **Bottom branch**: Process (No quality checks on left, Unclear procedures on right)

### Example 2: Complex Ishikawa with Nested Children

```jsonl
{"type":"ishikawa","id":"root","problem":"Low Product Quality","categories":[{"name":"People","items":[{"name":"Training","children":[{"name":"Lack of courses"},{"name":"Poor quality materials"}]},{"name":"High turnover"},{"name":"Low motivation"}]},{"name":"Process","items":[{"name":"Documentation incomplete"},{"name":"Lack of reviews"},{"name":"No standardization"}]},{"name":"Technology","items":[{"name":"Obsolete hardware"},{"name":"Software issues","children":[{"name":"No patches"},{"name":"Old versions"}]},{"name":"Lack of integration"}]},{"name":"Environment","items":[{"name":"Unstable temperature"},{"name":"High humidity"}]}]}
```

**Result:**
- **Top branches**: People, Process
- **Bottom branches**: Technology, Environment
- **Nested items**: "Training" and "Software issues" have sub-causes displayed hierarchically

### Example 3: Six Categories (6M Method)

```jsonl
{"type":"ishikawa","id":"root","problem":"Production Defects","categories":[{"name":"Man","items":[{"name":"Skill gaps"},{"name":"Fatigue"}]},{"name":"Method","items":[{"name":"Outdated procedures"},{"name":"No documentation"}]},{"name":"Machine","items":[{"name":"Poor maintenance"},{"name":"Old equipment"}]},{"name":"Material","items":[{"name":"Low quality suppliers"},{"name":"Inconsistent batches"}]},{"name":"Measurement","items":[{"name":"Uncalibrated tools"},{"name":"Inconsistent standards"}]},{"name":"Mother Nature","items":[{"name":"Temperature fluctuations"},{"name":"Humidity"}]}]}
```

**Result:**
- **Top branches** (3): Man, Method, Machine
- **Bottom branches** (3): Material, Measurement, Mother Nature

## Visual Layout

```
         [Item]─┐
                │
    [Item]──────┤     [Category]
                │          │
         [Item]─┘          │
                           │
═══════════════════════════╪═══════════════════ [Problem]
                           │
         [Item]─┐          │
                │          │
    [Item]──────┤     [Category]
                │
         [Item]─┘
```

## Best Practices

1. **Problem Statement**: Be specific and measurable (e.g., "Customer satisfaction below 80%" rather than "Poor service")

2. **Category Names**: Use standard frameworks when applicable:
   - **4M**: Man, Method, Machine, Material
   - **6M**: Add Measurement, Mother Nature (Environment)
   - **8P** (Service): People, Process, Place, Promotion, Product, Price, Physical Evidence, Positioning
   - Or create custom categories relevant to your domain

3. **Items**:
   - Keep descriptions concise (2-5 words)
   - Use specific, actionable causes
   - Nest related sub-causes using `children` for clarity

4. **Distribution**:
   - Aim for balanced categories (2-6 items per category)
   - The system automatically balances left/right distribution
   - Odd numbers work fine (e.g., 5 items = 3 left, 2 right)

5. **Nesting Depth**:
   - Use 1-2 levels of nesting for clarity
   - Avoid more than 3 levels (becomes hard to read)

## Common Use Cases

- **Manufacturing Quality**: Using 6M framework
- **Service Quality**: Using 8P framework
- **Project Issues**: Custom categories like Resources, Planning, Communication, Technology
- **Root Cause Analysis**: Any domain-specific categories
- **Problem Solving**: Brainstorming and organizing potential causes

## Notes

- All visual layout (positioning, spacing, connector lines) is handled automatically
- Categories are evenly distributed between top and bottom branches
- Items within categories are evenly distributed between left and right sides
- The diagram adapts to any number of categories and items