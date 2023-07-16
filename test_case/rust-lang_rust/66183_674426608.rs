java
public enum Planet {
    MERCURY (3.301e+23, 2.440e+6),
    VENUS   (4.868e+24, 6.052e+6),
    EARTH   (5.972e+24, 6.371e+6),
    MARS    (6.417e+23, 3.390e+6),
    JUPITER (1.898e+27, 6.991e+7),
    SATURN  (5.683e+26, 5.823e+7),
    URANUS  (8.681e+25, 2.536e+7),
    NEPTUNE (1.024e+26, 2.462e+7);

    private final double mass;
    private final double radius;

    Planet(double mass, double radius) {
        this.mass = mass;
        this.radius = radius;
    }

    // additional methods
}
