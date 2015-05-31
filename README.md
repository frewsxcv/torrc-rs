Torrc Parsing Crate
===================

This crate implements a simple parser for Tor configuration files.  It uses the
`nom` parser combinator crate for implementation.

## Gramar

```
conf -> __ settings_list __

settings_list -> <empty>
               | setting settings_list

setting -> name_value
         | name_quoted_value

name_value -> __ name value_list __

name_quoted_value -> __ name "\"" quoted_value_list "\"" __

name -> [a-zA-Z][a-zA-Z0-9]*

value_list -> <empty>
            | value value_list

quoted_value_list -> <empty>
                   | quoted_value quoted_value_list

value -> [^#\s\\]

quoted_value -> [^#\s]

__ -> <empty>
    | whitespace __
    | eol __
    | comment __

whitespace -> [\s\t]

eol -> "\r\n"
     | "\n"

comment -> "#"[^\n]*"\n"
```


