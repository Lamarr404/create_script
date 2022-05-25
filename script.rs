use std::fs::File;
use std::io::prelude::*;
use std::io::stdin;

fn main() -> std::io::Result<()> {

	println!("Choisissez le langage: ");
        println!("1) Bash ");
        println!("2) Python ");
        println!("3) Markdown ");
        println!("4) Rust ");

	let mut choix = String::new();

        stdin()
                .read_line(&mut choix)
                .expect("Erreur");

	let fchoix: i32 = choix.trim().parse().unwrap();

	if fchoix == 1 {

               println!("Entrez le nom du script Bash: ");

                let mut _name = String::new();

                stdin()
                        .read_line(&mut _name)
                        .expect("Erreur");

                let mut fname = _name.trim().to_owned();
                let ftype = ".sh";

                fname.push_str(&ftype);

                let mut file = File::create(fname)?;

                file.write_all(b"#!/bin/bash")?;
                Ok(())

	} else if fchoix == 2 {

		println!("Entrez le nom du script python3: ");

                let mut _name = String::new();

                stdin()
                        .read_line(&mut _name)
                        .expect("Erreur");

                let mut fname = _name.trim().to_owned();
                let ftype = ".py";

                fname.push_str(&ftype);

                let mut file = File::create(fname)?;

                file.write_all(b"#!/bin/env python3")?;
                Ok(())
	

        } else if fchoix == 3 { 

                println!("Entrez le nom du script markdown: ");

                let mut _name = String::new();

                stdin()
                        .read_line(&mut _name)
                        .expect("Erreur");

                let mut fname = _name.trim().to_owned();
                let ftype = ".md";

                fname.push_str(&ftype);

                let mut file = File::create(fname)?;

                file.write_all(b"#")?;
                Ok(())

        } else if fchoix == 4 { 

                println!("Entrez le nom du script Rust: ");

                let mut _name = String::new();

                stdin()
                        .read_line(&mut _name)
                        .expect("Erreur");

                let mut fname = _name.trim().to_owned();
                let ftype = ".sh";

                fname.push_str(&ftype);

                let mut file = File::create(fname)?;

                file.write_all(b"#!/bin/bash")?;
                Ok(())

	} else {


		println!("Veuillez choisir un script svp");
		panic!("Veuillez entrer un nombre entre 1 et 4 svp");
	}

	
}
