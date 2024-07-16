pub mod entity {
    
}

pub mod api {
    extern crate lettre;
    extern crate lettre_email;

    use lettre_email::Email;
    use lettre::smtp::authentication::Credentials;
    use lettre::{SmtpClient, Transport};

    pub fn send_email() {
        // 邮件接收人
        let email_receiver = "XuBaoPing@dbg.com.cn";
        // 邮件发送人
        let email_user = "2569158019@qq.com";
        // 服务器HOST
        //let smtp_server_host = "smtp.dbg.com.cn";
        let smtp_server_host = "smtp.qq.com";
        // 服务器授权码
        //let password = "*NR)-sZ?dk=n:ov}/D%O;1Jgg";
        let password = "pcxqiknqfyrtdjia";

        let email = Email::builder().to(email_receiver).from(email_user)
            .subject("测试邮件").text("测试内容").build().unwrap();

        let creds = Credentials::new(email_user.to_string(), password.to_string());
        let mut mailer = SmtpClient::new_simple(smtp_server_host).unwrap().credentials(creds).transport();

        let result = mailer.send(email.into());

        if result.is_ok() {
            println!("OK!");
        } else {
            println!("Error! Because: {:?}", result);
        }
        mailer.close();
        
    }
}
