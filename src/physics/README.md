# Physics
Notes about decisions in physics implementation.

## Collision Detection in Asteroids Game

### Problem Statement
In this game, objects (e.g., asteroids, bullets, player ship) are represented as concave polygons. The goal is to implement an efficient collision detection system to determine overlapping shapes.

### Alternatives Considered

1. **Separating Axis Theorem (SAT)**:
   - Efficient for convex polygons but requires concave decomposition.
   - Preprocessing adds complexity. **Not chosen.**
   - https://programmerart.weebly.com/separating-axis-theorem.html

2. **Bounding Volumes**:
   - Quick pre-check using circles or AABBs.
   - Insufficient for accurate collisions. **Useful as a filter, but not primary.**
   - https://en.wikipedia.org/wiki/Minimum_bounding_box#Axis-aligned_minimum_bounding_box

3. **Edge Intersection**:
   - Detects overlaps by checking if polygon edges cross.
   - Misses full containment. **Not sufficient for gameplay needs.**
   - https://en.wikipedia.org/wiki/Intersection_(geometry)#Two_line_segments

4. **Point-in-Polygon** (Chosen Approach):
   - Checks if vertices of one polygon lie inside another.
   - Simple to implement and handles full containment.
   - Misses edge-only collisions but acceptable for our use case.
   - https://en.wikipedia.org/wiki/Point_in_polygon

### Why Point-in-Polygon?
Point-in-Polygon is:
- Intuitive for concave shapes.
- Accurate for detecting overlaps or containment.
- Computationally efficient for moderate polygon sizes.

### Implementation
1. Use the **ray-casting method** for Point-in-Polygon:
   - Cast a ray from the test point and count edge intersections.
   - Odd count = point is inside.

2. Check all vertices of polygon A against polygon B (and vice versa for completeness).

### Complexity Analysis
- **Point-in-Polygon**: \(O(n)\) per test.
- **Overall Collision Detection**: \(O(m * n)\), where \(m\) and \(n\) are the vertex counts of the polygons.

### Future Work
- Add bounding volume checks (e.g., circles/AABBs) as a pre-filter.
- Explore hybrid methods if edge-only collisions become relevant.

### Conclusion
Point-in-Polygon balances simplicity and functionality, meeting our game’s needs for efficient and accurate collision detection.
