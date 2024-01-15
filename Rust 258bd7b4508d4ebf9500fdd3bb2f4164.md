# Rust

## Syntax

- All variables are immutable by default. mut key word can be used to make a variable immutable.
- let is used to define a variable and const for constant.
- An immutable variable can be redefined using let to another data type and this is called shadowing.
- Rust allows you to add underscores _ to literal values, to improve readability.
- println!() is for printing and use {} inside “ ” to print a variable as
    
    ```rust
    println!("{} is your number", num);
    ```
    
- // is used for single line comment and /*     */ for block comment.
- In rust, syntax for calling a method via an object or via a pointer to an object, in both cases dot operator is used. (Rust has automatic referencing and deferencing)

## Data Types

Primitives

- Signed Integer (i32, i64, i128)
- Unsigned Integer (u32, u64, u128)
- Floating point (f32, f64)
- Boolean (bool)
- Character (char)

Non Primitives

- Tuples (”Hello”, 5)
    - To access values either deconstruct and assign
    
    ```rust
    let (a,b) = datatuple;
    ```
    
    - Or use dot operator
    - 
    
    ```rust
    let a = datatuple.0;
    ```
    
- Array [1,2,3,4,5]

## Functions

- fn keyword can be used to define functions.
- As a naming convention, all function names are in lower case and use underscore for space.
- To specify a return type - > can be used after the parameter list.
- Functions can either return by using a return statement, or they can return implicitly by just putting the variable(or return value) in the last line. We omit the semicolon for last expression(or value)

## Control Flow

- if statement’s conditions doesn’t require paranthesis.
- If we put a number (or some expression not resulting in boolean) inside if condition then it will result in an error unlike javascript.
- We can have if else statements assigned to a let variable, to initialize it just as in the case of a ternary operator.
- loop keyword can be used to run an indefinite loop until a break statement is encountered.
- We can also have a loop statement assigned to a let variable and the let variable can be given whatever value is present after the break (here break is used like return keyword)
- There is also while loop and for loop available in rust.

## Ownership Model of Rust

![Untitled](Rust%20258bd7b4508d4ebf9500fdd3bb2f4164/Untitled.png)

### Stack and Heap

Note:-

1. Pushing data to stack is faster than allocating storage to data on heap as heap has to search for empty space.
2. Accessing data from stack is faster than accessing data from heap, as in heap you’ll have to follow the pointers where as usually data is directly accessible in stack (especially the primitive datatypes).
3. Non primitive data types’ pointer is stored on stack, whenever we want to store them in stack.

### Ownership Rules

1. Each value in rust has a variable that’s called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

When we copy a primitive datatype variable, a new copy is created Eg

```rust
let x: i32 = 5;
let y: i32 = x; //A new Copy is created
```

When we copy a non-primitive datatype variable, it neither does create a copy nor does it points to the same memory in heap(like in Java), btw this is called a shallow copy. Instead, it moves the data from first variable to the new variable and invalidates the first variable.

```rust
let s1: String = String::from("hello");
let s2: String = s1; //Move(not shallow copy)

println!("{}, world!", s1);
/* This will result in an error,
because s1 is invalidated and its data is moved */

//this creates a new copy in heap
let s3: String = s2.clone(); 
```

Note:- Passing a non primitive datatype variable to a functionn as parameter also moves the data and invalidates the original variable.

![Untitled](Rust%20258bd7b4508d4ebf9500fdd3bb2f4164/Untitled%201.png)

3rd line of main gives error, whereas last line of main runs without any error.

![Untitled](Rust%20258bd7b4508d4ebf9500fdd3bb2f4164/Untitled%202.png)

When gives_ownership() returns some_string, the ownership of somestring gets transfered as well and it can be used in main as s1.

![Untitled](Rust%20258bd7b4508d4ebf9500fdd3bb2f4164/Untitled%203.png)

takes_and_gives_back() first takes ownership from main function’s s2 and then returns the ownership to s3

### Problem with transfering ownership

![Untitled](Rust%20258bd7b4508d4ebf9500fdd3bb2f4164/Untitled%204.png)

Here you can see due to transfer of ownership we have to return a tuple for creating a simple length calculating function.

But references are a better solution to fix this problem.

### References

![Untitled](Rust%20258bd7b4508d4ebf9500fdd3bb2f4164/Untitled%205.png)

This is how we solve the above mentioned problem using a reference instead.

![Untitled](Rust%20258bd7b4508d4ebf9500fdd3bb2f4164/Untitled%206.png)

S is a reference which points to s1 and s1 in turn points to a memory location in heap.

Note:- 

1. References don’t take ownership of the underlying value.
2. References are immutable by default.
3. In order to have a mutable reference we use mut keyword for the original variable (ie s1 in the given example) and also add mut keyword to the reference (ie s in the given example) 

![Untitled](Rust%20258bd7b4508d4ebf9500fdd3bb2f4164/Untitled%207.png)

Limitation of mutable reference:

- You can have only one mutable reference to a piece of data in a particular scope.

![Untitled](Rust%20258bd7b4508d4ebf9500fdd3bb2f4164/Untitled%208.png)

- You cant have a mutable reference to a piece of data if an immutable reference to it already exists.

![Untitled](Rust%20258bd7b4508d4ebf9500fdd3bb2f4164/Untitled%209.png)

Note:- You can have multiple immutable references to a piece of data.

Note:- Scope of a reference starts at where it is first introduced and ends where it is last used.

![Untitled](Rust%20258bd7b4508d4ebf9500fdd3bb2f4164/Untitled%2010.png)

This means this is possible.

![Untitled](Rust%20258bd7b4508d4ebf9500fdd3bb2f4164/Untitled%2011.png)

Rust prevents us from getting a dangling reference(reference to an invalid memory). Here s is defined within the scope of dangle() and when its reference is pased to main, its ownership is not and by then the s is deallocated the memory. Hence rust gives an error to prevent this memory safety issue.

### Slices

Slices let you reference a contiguous sequence of elements within a collection instead of referencing the entire collection. Slices don’t take ownership just like references. 

![Untitled](Rust%20258bd7b4508d4ebf9500fdd3bb2f4164/Untitled%2012.png)

Here hello is a slice of string, it points to starting half and world is another slice pointing to the other half starting from index 6.

### Struct

![Untitled](Rust%20258bd7b4508d4ebf9500fdd3bb2f4164/Untitled%2013.png)