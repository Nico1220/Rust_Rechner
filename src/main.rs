
fn main() {
    let mut z1:f64 = 4.0;
    let mut z2:f64 = 2.0;
    let mut rechntyp = "*";
    let mut texteingabe = "y";

    println!("Zahl eingeben:");
    println!("Rechentyp eingeben :");
    println!("Zahl eingeben:");

    if "+" == &*rechntyp{
        println!("Ergebnis: {}", z1 + z2);
    }
    else if "-" == &*rechntyp{
        println!("Ergebnis: {}", z1 - z2);
    }
    else if "*" == &*rechntyp{
        println!("Ergebnis: {}", z1*z2);
    }
    else if "/" == &*rechntyp{
        println!("Ergebnis: {}", z1/z2);
    }
    else{
        println!("Fehlerhafte Eingabe")
    }

    println!("Beenden [y/n]:");



}