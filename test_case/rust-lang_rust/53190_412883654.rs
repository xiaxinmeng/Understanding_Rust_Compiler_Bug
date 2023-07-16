
        if !licenseck(file, &contents) {
            tidy_error!(bad, "{}: incorrect license", file.display());
        }
