use icondata as i;

pub static PROJECTS_DATA: &str = r#"
    [
        {
            "title": "CLLM",
            "description": "Empower your CLI experience with a command search tool driven by LLM magic!",
            "tags": ["Rust", "CLI", "LLM"],
            "image": "./cllm.png"
        },
        {
            "title": "Teacher's Pick",
            "description": "Korean high school Korean language subject problem solving service using forgetting curve and ai.",
            "tags": ["Mobile", "AI", "Korean", "Flutter", "NLP"],
            "image": "./teachers_pick.png"
        },
        {
            "title": "Style Shift",
            "description": "Image Style Transfer using Neural Networks",
            "tags": ["Computer Vision", "Neural Networks", "AI"],
            "image": "./style_shift.png"
        }
    ]
    "#;

pub static SKILLS_DATA: &[&str] = &[
    "C",
    "C++",
    "Python",
    "Rust",
    "Java",
    "JavaScript",
    "TypeScript",
    "PyTorch",
    "Lightning",
    "Numpy",
    "Pandas",
    "Scikit-learn",
    "React",
    "Node.js",
    "Nest.js",
    "Express",
    "Django",
    "Flask",
    "FastAPI",
    "Spring Boot",
    "Leptos",
    "WebAssembly",
    "MySQL",
    "PostgreSQL",
    "SQlite",
    "Docker",
    "Kubernetes",
    "AWS",
    "GCP"
];

pub struct ExperienceData<'a> {
    pub title: &'a str,
    pub location: &'a str,
    pub description: &'a str,
    pub icon: i::Icon,
    pub date: &'a str,
}

pub static EXPERIENCES_DATA : &[ExperienceData; 3] = &[
    ExperienceData {
        title: "Student",
        location: "Hanyang University, Seoul, South Korea",
        description: "Bachelor of Computer Science",
        icon: i::BsBook,
        date: "2019 - 2025",
    },
    ExperienceData {
        title: "LLM Engineer",
        location: "Seoul, Korea",
        description: "Development of an LLM for Malware Detection and Annotation",
        icon: i::BsGear,
        date: "2024",
    },
    ExperienceData {
        title: "Software Engineer",
        location: "ZER01NE Incubation at Hyundai Motor Company", 
        description: "Development of AI-based safety sensors for collaborative robots in smart factories",
        icon: i::BsSuitcaseLg,
        date: "2024",
    }
];