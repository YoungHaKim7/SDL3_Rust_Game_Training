# Explanation of changes made:

- 1. The `draw_points` method expects a slice of `FPoint`, but you're providing a slice of 
`Point`. To fix this, we iterate over each point in the vector and use the `draw_point` 
method.

- 2. There's no `inflate` method for the `Rect` struct in SDL3. Instead, we manually 
calculate the inner rectangle by adding or subtracting from its x, y, width, and height.

- 3. We've added a loop to continuously render frames until the window is closed.

- 4. The canvas needs to be cleared at the start of each frame to avoid drawing on top of 
previous frames. This is done using the `clear` method.

- 5. To handle events properly, we use a loop that polls for events and handles them 
accordingly. If the user closes the window, the program exits cleanly.

