# How to use beautiful_output

### the print function accepts four arguments
1 . Input  
2 . padding left  
3 . padding rigth  
4 . message    
5 . border vertical style   
6 . border horizontal style     
7 . border corner style 


```
[dependencies]
beautiful_output = "0.1.3"
```

## Example

**Code :**

```rust
use beautiful_output::print

print("Hello World!",3,7,"Output :-","-","|","+");
```


**Output :**
```
 Output :-
 +----------------------+
 |   Hello World!       |
 +----------------------+
```