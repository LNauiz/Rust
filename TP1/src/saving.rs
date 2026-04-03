use serde_json::{Value, from_str, to_string_pretty};
use csv;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Seek, SeekFrom};
use std::error::Error;
/*
pub fn lire_fichier_json(nom_fichier: &str) -> Result<String, Box<dyn std::error::Error>>{
    let contenu = fs::read_to_string(nom_fichier)?;
    Ok(contenu)
}

 */
/*
pub fn sauvegarder_resultats(resultats: &str, nom_fichier: &str,exist:bool,format :&str)-> Result<(), Box<dyn std::error::Error>> {
    if format=="json"{
        if exist == false {
            //let fichier = File::create(format!("{}", nom_fichier))?;
            let objet_json = json!({"msg": resultats});
            let json_array = format!("[{}]", objet_json);
            fs::write(nom_fichier, json_array)?;
        } else {
            // Lire le contenu existant
            let contenu = lire_fichier_json(nom_fichier)?;

            // Désérialiser le contenu en un tableau JSON
            let mut array: Vec<Value> = from_str(&contenu)?;

            // Ajouter le nouveau résultat au tableau
            let objet_json = json!({"msg": resultats});
            array.push(objet_json);

            // Sérialiser le tableau JSON mis à jour
            let json = to_string_pretty(&array)?;

            // Écrire le contenu mis à jour dans le fichier
            fs::write(nom_fichier, json)?;
        }
        Ok(())
    }
    else if format=="csv" {
        if exist == false {
            let fichier = File::create(nom_fichier)?;
            let mut writer = csv::Writer::from_writer(nom_fichier);
            writer.serialize(resultats)?;
            writer.flush()?;

        } else {
            // Lire le contenu existant
            let contenu = lire_fichier_json(nom_fichier)?;

            // Désérialiser le contenu en un tableau JSON
            let mut array: Vec<Value> = from_str(&contenu)?;

            let mut writer = csv::Writer::from_writer(nom_fichier);

            // Ajouter le nouveau résultat au tableau
            let objet_csv = json!({"msg": resultats});
            array.push(objet_csv);

            // Sérialiser le tableau JSON mis à jour
            let csv = to_string_pretty(&array)?;

            // Écrire le contenu mis à jour dans le fichier
            writer.serialize(csv)?;
            writer.flush()?;
        }
        Ok(())
    }
    else{
        Err("Format non supporté. Utilisez 'json' ou 'csv'.".into())
    }
}

*/


// Fonction pour sauvegarder une unique chaîne de caractères dans un fichier JSON ou CSV
pub fn sauvegarder_resultats2(resultat: &str, format: &str, nom_fichier: &str, append: bool) -> Result<(), Box<dyn Error>> {
    match format.to_lowercase().as_str() {
        "json" => {
            let path = format!("{}", nom_fichier);
            if !append {
                // Créer un nouveau fichier avec un tableau JSON contenant un seul objet
                let objet_json = serde_json::json!([{"valeur": resultat}]);
                let fichier = File::create(&path)?;
                serde_json::to_writer_pretty(fichier, &objet_json)?;
            } else {
                // Lire le contenu existant
                let contenu = std::fs::read_to_string(&path)?;

                // Désérialiser le contenu en un tableau JSON
                let mut array: Vec<Value> = if contenu.trim().is_empty() {
                    Vec::new()
                } else {
                    from_str(&contenu)?
                };

                // Ajouter le nouvel objet JSON au tableau
                array.push(serde_json::json!({"valeur": resultat}));

                // Sérialiser le tableau JSON mis à jour
                let json = to_string_pretty(&array)?;

                // Écrire le contenu mis à jour dans le fichier
                std::fs::write(&path, json)?;
            }
        }
        "csv" => {
            let path = format!("{}", nom_fichier);
            let fichier = OpenOptions::new()
                .write(true)
                .append(append)
                .create(true)
                .open(&path)?;

            let mut writer = csv::Writer::from_writer(fichier);

            if !append {
                // Écrire l'en-tête
                writer.write_record(&["valeurs"])?;
            }

            // Écrire la chaîne de caractères comme une ligne dans le CSV
            writer.write_record(&[resultat])?;

            writer.flush()?;
        }
        _ => return Err("Format non supporté. Utilisez 'json' ou 'csv'.".into()),
    }
    Ok(())
}


fn main() {
    let _ =sauvegarder_resultats2("Test Création","test.json","json",false);
    let _ =sauvegarder_resultats2("Test Edition","test.json","json",true);

}
/*
let mut file =lire_fichier_json(nom_fichier)?.to_string();
file.push_str(resultats);
let mut json = serde_json::to_string_pretty(&file)?;
json.push_str(" \n ");
fs::write(nom_fichier, json)?;
Ok(())
*/