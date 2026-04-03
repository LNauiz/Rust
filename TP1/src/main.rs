use clap::Parser;
use TP1::saving;

#[derive(Parser)]
struct Args  {
    #[arg(long)]
    pcap: Option<String>,

    #[arg(long, conflicts_with("pcap"))]
    interface: Option<String>,

    #[arg(long)]
    cards:bool,

    #[arg(long)]
    filter: Option<String>,

    #[arg(long,default_value("10"))]
    packet_count: u32,

    #[arg(long,default_value("JSON"))]
    output_format: String,

    #[arg(long,default_value("results.json"))]
    output_file: String,
}





fn main() {
    let args = Args::parse();
    println!("Hello, world!");
    let file = &args.pcap.unwrap(); //On récupère le nom du fichier
    let output= &args.output_file;
    let format = &args.output_format;
    let _=saving::sauvegarder_resultats2("",format,output,false); //création du fichier d'output

    let mut cap = pcap::Capture::from_file(file).expect("failed to open file");
    let tableau=["Version","ID FR","","Latitude","Longitude","Altitude AWSL","Altitude AGL","Latitude Takeoff",
        "Longitude Takeoff","Horizontal Speed", "Heading"];
    let mut pack_nb=0;
    while let Ok(packet) = cap.next_packet() {
        pack_nb+=1;
        let data = packet.data;

        if data.len()<4 {continue} // On verifie que le pacquet contient plus de 4 éléments
        // Sinon on passe à l'élément suivant -> pas de données.
        let radiotap_len = u16::from_le_bytes([data[2], data[3]]) as usize;
        //La longueur du header se trouve dans les bytes 2 et 3 cf screenshot
        let mac_header_start = radiotap_len; // le header de mac commence au bit après le header radiotap
        if data.len() < mac_header_start+24 {continue} // si l'adresse MAC ne fait pas 24 bytes on passe au apcket suivant

        // on récupère le type_bit et le subtype bit afin de vérifier que le paquet est un beacon
        let frame_control = data[mac_header_start];
        let type_bit = (frame_control >> 2) & 0x03;
        let subtype_bit = (frame_control >> 4) & 0x0F;
        let mut mac_addr:String="".to_string();
        if type_bit==0 && subtype_bit==8 {
            // l'adresse MAC se trouve dans les bits 16 à 21 du frame control
            let bssid_start = mac_header_start + 16;
            mac_addr = format!(
                "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                data[bssid_start], data[bssid_start + 1], data[bssid_start + 2],
                data[bssid_start + 3], data[bssid_start + 4], data[bssid_start + 5]
            ); // Récupération de l'adresse MAC
        }

        // 4. Saut du Management Header (taille fixe de 12 octets après les 24 octets du MAC Header)
        let tlv_section_start = mac_header_start + 24 + 12;
        let mut current_pos = tlv_section_start;

        // 5. Lecture des champs TLV pour trouver le SSID (Type 0x00)
        let mut ssid = String::from("<Inconnu>");

        while current_pos + 2 <= data.len() {
            let tlv_type = data[current_pos];
            let tlv_len = data[current_pos + 1] as usize;

            if current_pos + 2 + tlv_len > data.len() { break; }

            if tlv_type == 0x00 { // Type SSID
                let ssid_bytes = &data[current_pos + 2 .. current_pos + 2 + tlv_len];
                if ssid_bytes.is_empty() {
                    ssid = String::from("<Hidden SSID>");
                } else {
                    ssid = String::from_utf8_lossy(ssid_bytes).into_owned();
                }
                break; // On a trouvé le SSID, on peut arrêter de chercher dans les TLV
            }

            // Passer au TLV suivant : Type (1) + Length (1) + Value (L)
            current_pos += 2 + tlv_len;
        }
        if ssid == "THESEUS_DAO_1" {
            println!("{:<20} | {:<20}", mac_addr, ssid);
            let _=saving::sauvegarder_resultats2(&mac_addr,format,output,true);
            let _=saving::sauvegarder_resultats2(&ssid,format,output,true);

        }
        current_pos= current_pos + 2 + (data[current_pos + 1] as usize);
        let vendor_spec=data[current_pos];
        //let tag_length=data[current_pos+1] as usize;
        current_pos += 5;
        if vendor_spec==0xdd {
            for i in 1..11 {
                if data[current_pos + 1]== i {
                    let len_data=data[current_pos + 2] as usize;
                    let extract = &data[current_pos + 3 .. current_pos + 3+len_data];
                    if i==2{
                        let mut msg2 =String::new();
                        for j in 0..extract.len() {
                            msg2.push(extract[j] as char);
                        }
                        //println!("{} : {:?}",tableau[(i-1) as usize],msg2);
                        let txt= format!("{} : {:?}", tableau[(i - 1) as usize], msg2);
                        //println!("{}",txt);
                        let _=saving::sauvegarder_resultats2(&msg2,format,output,true);

                    }
                    else {
                        let mut msg=String::new();
                        //println!("{}",extract[extract.len()-1]);
                        if (extract[extract.len()-1]>127) {
                            msg.push_str("-"); //Permet de régler manuellement le bit de signe.
                        }
                            for j in 0..extract.len() {
                            msg.push_str(&format!("{:02x}", extract[extract.len()-1-j]));
                        }
                        let msg_hexa=i64::from_str_radix(&msg, 16).unwrap();
                        let txt= format!("{} : {:?}", tableau[(i - 1) as usize], msg_hexa);
                        //println!("{}",txt);
                        let _=saving::sauvegarder_resultats2(&txt,format,output,true);

                    }
                    current_pos += 2 + len_data;
                }

            }
        }

        //let mut military :String="".to_string();
        //military = format!(
        //    "{:02x}:{:02x}:{:02x}",
        //     data[current_pos], data[current_pos +1], data[current_pos + 2]
        //);
    }
}