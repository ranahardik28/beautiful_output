# How to use beautiful_output

### the print function accepts four arguments
1 . Input  
2 . padding left  
3 . padding rigth  
4 . message  


```
[dependencies]
beautiful_output = "0.1.2"
```

## Example

**Code :**

```rust
use beautiful_output::print

print("Hello World!",3,7,"Output :-");
```


**Output :**
```
 Output :-
 +----------------------+
 |   Hello World!       |
 +----------------------+
```