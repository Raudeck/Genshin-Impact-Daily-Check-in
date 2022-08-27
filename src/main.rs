mod checkin;

fn main() {
    let checkin = checkin::AccountInfo::new("account_id=YOUR_ACCOUNT_ID; cookie_token=YOUT_COOKIE".to_string());
    checkin.check_in().unwrap();
}