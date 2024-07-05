use std::process::Command;

pub fn creer_clef(clef: &str) {
    let bash_script = format!(
        r#"
        echo -n "{}" > aes_key.txt
        xxd -r -p aes_key.txt > aes_key.bin
        sudo pkcs11-tool --module /usr/lib/softhsm/libsofthsm2.so -l -p 1111 --write-object aes_key.bin --type secrkey --key-type AES:32 --id 00 --label "clef AES"
        "#,
        clef
    );

    let _output = Command::new("bash")
    .arg("-c")
    .arg(&bash_script)
    .output();
}

pub fn supr_clef() {
    let bash_script = r#"sudo pkcs11-tool --module /usr/lib/softhsm/libsofthsm2.so -l -p 1111 -b --type secrkey --id 00"#;

    let _output = Command::new("bash")
    .arg("-c")
    .arg(&bash_script)
    .output();
}