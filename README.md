# rust-exercise-1

1. What is the differences between pointer type: &, &mut, Box, Rc, Arc, *const, *mut

```
&: immutable reference
 - doesn't take ownership of the data
 - lifetime is checked by the compiler
 - borrow can happen indefinitely in the same scope
 - exclusive with &mut

&mut: mutable reference
 - doesn't take ownership of the data
 - lifetime is checked by the compiler
 - can be borrowed only once in the same scope
 - exclusive with &

Box: unique pointer
 - take ownership of the data (and move the data from stack to heap)
 - single ownership
 - free the memory automatically after dropped

Rc: referenced-counted pointer
 - take ownership of the data (and move the data from stack to heap)
 - multiple ownership (count ownership)
 - immutable borrowed (if not using inner mutability)
 - free the memory when the reference counting reaches 0
 - can cause memory leak in reference cycle (consider using weak)

Arc: atomic RC
 - same as RC except that it's atomic
 - expensive than RC

*const: immutable raw pointer
 - unsafe to dereference
 - lifetime and ownership is not working
 - cannot be reassigned
 - nullable


*mut: mutable raw pointer
 - unsafe to dereference
 - lifetime and ownership is not working
 - can be reassigned
 - nullable
```
