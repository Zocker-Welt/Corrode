> [!IMPORTANT]  
> This is a WIP project

> [!NOTE]  
> This language is for people who are new to coding and it's highly unoptimized


<h1 align="center">
    Corrode 0.4.3
</h1>

<p align="center">
    An interpreter written in rust
</p>

<h1 align="center">
    Usage
</h1>

<p align="left">
    Open the shell (make sure you are in the directory where the executable is located)
</p>

```bash
./corrode
```

<p align="left">
    Run code from file (make sure you are in the directory where the executable is located)
</p>

```bash
./corrode [file]
```

<h1 align="center">
    Documentation
</h1>

<h3 align="center">
    Quick tutorial
</h1>

```js
print expression;   // Write to io a value
let name = value;   // Create a variable and instantly give it a value
let name;           // Create a variable (the variable's value is null)
new_var = value;    // Create a variable (must have a value)
print name = value  // Variable assignment is an expression (not a statement). It returns the new value 
```

<h3 align="center">
    Data types
</h3>

```js
name:           Number
description:    Floating point number
example:        31.4
```

```js
name:           String
description:    Text value
example:        "Hello, world!"
```

```js
name:           bollean
description:    bollean value
examples:        true, false
```

```js
name:           null
description:    null value
example:        null
```

<h3 align="center">
    Keywords
</h3>

```js
and
or

true
false

if
else
class
self

fn
return

for
while

null

print

super

let
```

<h3 align="center">
    Operators
</h3>

```js
+ - * / ( )
```
