# Type-Driven API Design in Rust
From the recording of the presentation of the same name by Will Crichton at the Strange Loop Conference in St. Louis, USA.

I used different names for the type member in the last version of main.rs. I also preferred to call `last_element` or `num_elements` the `len` of the progress bar. 

Each of the files enumerated 1 to 5 has the code at different points of the talk. 


1. `1.main.simple.rs` at 19:23: https://youtu.be/bnnacleqg6k?si=poTF-qLtRTntr3ia&t=1173
2. `2.main.dot.rs` at 22:22: https://youtu.be/bnnacleqg6k?si=5RlVL4LKkEhGbxqk&t=1342 
3. `3.main.last_elem.rs` at 29:31: https://youtu.be/bnnacleqg6k?si=oDqjpGaDNm3425tf&t=1771
4. `4.main.bookends.rs` at 32:53: https://youtu.be/bnnacleqg6k?si=lldG9QJp-PymzJUa&t=1973
5. `5.main.type_state.rs` at 37:58: https://youtu.be/bnnacleqg6k?si=WWCLTOUocVLsqv6R&t=2278



# Type State

In `5.main.type_state.rs` there are two states for an instance of Progress to be in: Unbound and Bound.

```mermaid
stateDiagram-v2
    [*] --> Unbound
    Unbound --> Bound
    Bound --> [*]
    Unbound --> [*]
```

# References
1. https://youtu.be/bnnacleqg6k?si=1N58X2fte4oZB19M
