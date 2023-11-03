# templater

Pulls from another remote repository and copies the subfolder related to the template.  

Example template repository subfolder structure:  
```
└── cpp
    ├── main.cpp
    └── test
```

Example command: `templater cpp`  

## To install

- Create remote repository 
- `echo 'YOUR_TEMPLATE_REPO_URL' >  src/template_url`
- `cargo install --path .`