use std::process::Command;
fn main(){

    tonic_build::configure()
        .out_dir("src/pb")
        .compile(&["protos/reservation.proto"],&["protos"])
        .unwrap();

    Command::new("cargo")
        .args(&["fmt"])
        .current_dir("src/pb")
        .output()
        .unwrap();




}
