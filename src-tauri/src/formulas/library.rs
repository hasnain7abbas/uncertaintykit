use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormulaEntry {
    pub name: String,
    pub category: String,
    pub formula: String,
    pub description: String,
    pub variables: Vec<FormulaVariable>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormulaVariable {
    pub name: String,
    pub description: String,
    pub unit: String,
    pub typical_value: Option<f64>,
}

pub fn get_formula_library() -> Vec<FormulaEntry> {
    vec![
        // === MECHANICS ===
        FormulaEntry {
            name: "Velocity".into(),
            category: "Mechanics".into(),
            formula: "d / t".into(),
            description: "Average velocity from distance and time".into(),
            variables: vec![
                FormulaVariable { name: "d".into(), description: "Distance".into(), unit: "m".into(), typical_value: None },
                FormulaVariable { name: "t".into(), description: "Time".into(), unit: "s".into(), typical_value: None },
            ],
        },
        FormulaEntry {
            name: "Kinetic Energy".into(),
            category: "Mechanics".into(),
            formula: "0.5 * m * v^2".into(),
            description: "Kinetic energy of a moving object".into(),
            variables: vec![
                FormulaVariable { name: "m".into(), description: "Mass".into(), unit: "kg".into(), typical_value: None },
                FormulaVariable { name: "v".into(), description: "Velocity".into(), unit: "m/s".into(), typical_value: None },
            ],
        },
        FormulaEntry {
            name: "Gravitational PE".into(),
            category: "Mechanics".into(),
            formula: "m * g * h".into(),
            description: "Gravitational potential energy".into(),
            variables: vec![
                FormulaVariable { name: "m".into(), description: "Mass".into(), unit: "kg".into(), typical_value: None },
                FormulaVariable { name: "g".into(), description: "Gravitational acceleration".into(), unit: "m/s\u{00B2}".into(), typical_value: Some(9.81) },
                FormulaVariable { name: "h".into(), description: "Height".into(), unit: "m".into(), typical_value: None },
            ],
        },
        FormulaEntry {
            name: "Projectile Range".into(),
            category: "Mechanics".into(),
            formula: "v^2 * sin(2 * theta) / g".into(),
            description: "Horizontal range of a projectile".into(),
            variables: vec![
                FormulaVariable { name: "v".into(), description: "Launch speed".into(), unit: "m/s".into(), typical_value: None },
                FormulaVariable { name: "theta".into(), description: "Launch angle".into(), unit: "rad".into(), typical_value: None },
                FormulaVariable { name: "g".into(), description: "Gravitational acceleration".into(), unit: "m/s\u{00B2}".into(), typical_value: Some(9.81) },
            ],
        },
        FormulaEntry {
            name: "Centripetal Acceleration".into(),
            category: "Mechanics".into(),
            formula: "v^2 / r".into(),
            description: "Centripetal acceleration in circular motion".into(),
            variables: vec![
                FormulaVariable { name: "v".into(), description: "Tangential speed".into(), unit: "m/s".into(), typical_value: None },
                FormulaVariable { name: "r".into(), description: "Radius".into(), unit: "m".into(), typical_value: None },
            ],
        },
        FormulaEntry {
            name: "Simple Pendulum Period".into(),
            category: "Mechanics".into(),
            formula: "2 * pi * sqrt(L / g)".into(),
            description: "Period of a simple pendulum (small angle)".into(),
            variables: vec![
                FormulaVariable { name: "L".into(), description: "Pendulum length".into(), unit: "m".into(), typical_value: None },
                FormulaVariable { name: "g".into(), description: "Gravitational acceleration".into(), unit: "m/s\u{00B2}".into(), typical_value: Some(9.81) },
            ],
        },
        FormulaEntry {
            name: "Spring Potential Energy".into(),
            category: "Mechanics".into(),
            formula: "0.5 * k * x^2".into(),
            description: "Elastic potential energy in a spring".into(),
            variables: vec![
                FormulaVariable { name: "k".into(), description: "Spring constant".into(), unit: "N/m".into(), typical_value: None },
                FormulaVariable { name: "x".into(), description: "Extension".into(), unit: "m".into(), typical_value: None },
            ],
        },
        // === ELECTRICITY ===
        FormulaEntry {
            name: "Ohm's Law (Power)".into(),
            category: "Electricity".into(),
            formula: "V^2 / R".into(),
            description: "Power dissipated in a resistor".into(),
            variables: vec![
                FormulaVariable { name: "V".into(), description: "Voltage".into(), unit: "V".into(), typical_value: None },
                FormulaVariable { name: "R".into(), description: "Resistance".into(), unit: "\u{03A9}".into(), typical_value: None },
            ],
        },
        FormulaEntry {
            name: "RC Time Constant".into(),
            category: "Electricity".into(),
            formula: "R * C".into(),
            description: "Time constant of an RC circuit".into(),
            variables: vec![
                FormulaVariable { name: "R".into(), description: "Resistance".into(), unit: "\u{03A9}".into(), typical_value: None },
                FormulaVariable { name: "C".into(), description: "Capacitance".into(), unit: "F".into(), typical_value: None },
            ],
        },
        FormulaEntry {
            name: "Capacitor Energy".into(),
            category: "Electricity".into(),
            formula: "0.5 * C * V^2".into(),
            description: "Energy stored in a capacitor".into(),
            variables: vec![
                FormulaVariable { name: "C".into(), description: "Capacitance".into(), unit: "F".into(), typical_value: None },
                FormulaVariable { name: "V".into(), description: "Voltage".into(), unit: "V".into(), typical_value: None },
            ],
        },
        // === OPTICS ===
        FormulaEntry {
            name: "Thin Lens Equation (f)".into(),
            category: "Optics".into(),
            formula: "(u * v) / (u + v)".into(),
            description: "Focal length from object and image distances".into(),
            variables: vec![
                FormulaVariable { name: "u".into(), description: "Object distance".into(), unit: "cm".into(), typical_value: None },
                FormulaVariable { name: "v".into(), description: "Image distance".into(), unit: "cm".into(), typical_value: None },
            ],
        },
        FormulaEntry {
            name: "Snell's Law (\u{03B8}\u{2082})".into(),
            category: "Optics".into(),
            formula: "asin(n1 * sin(theta1) / n2)".into(),
            description: "Refracted angle from Snell's law".into(),
            variables: vec![
                FormulaVariable { name: "n1".into(), description: "Refractive index (medium 1)".into(), unit: "".into(), typical_value: Some(1.0) },
                FormulaVariable { name: "theta1".into(), description: "Incident angle".into(), unit: "rad".into(), typical_value: None },
                FormulaVariable { name: "n2".into(), description: "Refractive index (medium 2)".into(), unit: "".into(), typical_value: Some(1.5) },
            ],
        },
        // === WAVES ===
        FormulaEntry {
            name: "Wave Speed".into(),
            category: "Waves".into(),
            formula: "f * lambda".into(),
            description: "Wave speed from frequency and wavelength".into(),
            variables: vec![
                FormulaVariable { name: "f".into(), description: "Frequency".into(), unit: "Hz".into(), typical_value: None },
                FormulaVariable { name: "lambda".into(), description: "Wavelength".into(), unit: "m".into(), typical_value: None },
            ],
        },
        // === CRYSTALLOGRAPHY ===
        FormulaEntry {
            name: "Bragg's Law (d-spacing)".into(),
            category: "Crystallography".into(),
            formula: "n * lambda / (2 * sin(theta))".into(),
            description: "Interplanar d-spacing from Bragg diffraction".into(),
            variables: vec![
                FormulaVariable { name: "n".into(), description: "Diffraction order".into(), unit: "".into(), typical_value: Some(1.0) },
                FormulaVariable { name: "lambda".into(), description: "X-ray wavelength".into(), unit: "\u{00C5}".into(), typical_value: Some(1.5406) },
                FormulaVariable { name: "theta".into(), description: "Bragg angle".into(), unit: "rad".into(), typical_value: None },
            ],
        },
        FormulaEntry {
            name: "Scherrer Equation".into(),
            category: "Crystallography".into(),
            formula: "K * lambda / (B * cos(theta))".into(),
            description: "Crystallite size from XRD peak broadening".into(),
            variables: vec![
                FormulaVariable { name: "K".into(), description: "Shape factor".into(), unit: "".into(), typical_value: Some(0.9) },
                FormulaVariable { name: "lambda".into(), description: "X-ray wavelength".into(), unit: "\u{00C5}".into(), typical_value: Some(1.5406) },
                FormulaVariable { name: "B".into(), description: "FWHM of peak".into(), unit: "rad".into(), typical_value: None },
                FormulaVariable { name: "theta".into(), description: "Bragg angle".into(), unit: "rad".into(), typical_value: None },
            ],
        },
        // === THERMAL ===
        FormulaEntry {
            name: "Specific Heat (Q)".into(),
            category: "Thermal".into(),
            formula: "m * c * dT".into(),
            description: "Heat energy from mass, specific heat, and temperature change".into(),
            variables: vec![
                FormulaVariable { name: "m".into(), description: "Mass".into(), unit: "kg".into(), typical_value: None },
                FormulaVariable { name: "c".into(), description: "Specific heat capacity".into(), unit: "J/(kg\u{00B7}K)".into(), typical_value: None },
                FormulaVariable { name: "dT".into(), description: "Temperature change".into(), unit: "K".into(), typical_value: None },
            ],
        },
        FormulaEntry {
            name: "Ideal Gas (PV=nRT)".into(),
            category: "Thermal".into(),
            formula: "n * R * T / V".into(),
            description: "Pressure of an ideal gas".into(),
            variables: vec![
                FormulaVariable { name: "n".into(), description: "Amount of substance".into(), unit: "mol".into(), typical_value: None },
                FormulaVariable { name: "R".into(), description: "Gas constant".into(), unit: "J/(mol\u{00B7}K)".into(), typical_value: Some(8.314) },
                FormulaVariable { name: "T".into(), description: "Temperature".into(), unit: "K".into(), typical_value: None },
                FormulaVariable { name: "V".into(), description: "Volume".into(), unit: "m\u{00B3}".into(), typical_value: None },
            ],
        },
        // === GRAVITATION ===
        FormulaEntry {
            name: "Gravitational Force".into(),
            category: "Gravitation".into(),
            formula: "G * m1 * m2 / r^2".into(),
            description: "Newton's law of universal gravitation".into(),
            variables: vec![
                FormulaVariable { name: "G".into(), description: "Gravitational constant".into(), unit: "N\u{00B7}m\u{00B2}/kg\u{00B2}".into(), typical_value: Some(6.674e-11) },
                FormulaVariable { name: "m1".into(), description: "Mass 1".into(), unit: "kg".into(), typical_value: None },
                FormulaVariable { name: "m2".into(), description: "Mass 2".into(), unit: "kg".into(), typical_value: None },
                FormulaVariable { name: "r".into(), description: "Distance".into(), unit: "m".into(), typical_value: None },
            ],
        },
        // === MODERN PHYSICS ===
        FormulaEntry {
            name: "de Broglie Wavelength".into(),
            category: "Modern Physics".into(),
            formula: "h / (m * v)".into(),
            description: "de Broglie wavelength of a particle".into(),
            variables: vec![
                FormulaVariable { name: "h".into(), description: "Planck's constant".into(), unit: "J\u{00B7}s".into(), typical_value: Some(6.626e-34) },
                FormulaVariable { name: "m".into(), description: "Particle mass".into(), unit: "kg".into(), typical_value: None },
                FormulaVariable { name: "v".into(), description: "Velocity".into(), unit: "m/s".into(), typical_value: None },
            ],
        },
        FormulaEntry {
            name: "Density".into(),
            category: "Material Properties".into(),
            formula: "m / V".into(),
            description: "Density from mass and volume".into(),
            variables: vec![
                FormulaVariable { name: "m".into(), description: "Mass".into(), unit: "kg".into(), typical_value: None },
                FormulaVariable { name: "V".into(), description: "Volume".into(), unit: "m\u{00B3}".into(), typical_value: None },
            ],
        },
    ]
}
