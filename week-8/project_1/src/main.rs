
fn main() {
   
    let mut staff_member = ("Lawyer", 6);

    
    println!(
        "Profession = {}, Experience = {} years",
        staff_member.0, staff_member.1
    );

   
    staff_member = ("Lawyer", 9);

   
    println!(
        "Profession = {}, Experience = {} years",
        staff_member.0, staff_member.1
    );

    
    let aps_level = get_aps_level(staff_member);
    println!("The APS Level is: {}", aps_level);
}


fn get_aps_level(staff: (&str, usize)) -> &'static str {
    match staff.0 {
        "OfficeAdministrator" => match_office_admin(staff.1),
        "Academic" => match_academic(staff.1),
        "Lawyer" => match_lawyer(staff.1),
        "Teacher" => match_teacher(staff.1),
        _ => "Invalid Profession",
    }
}


fn match_office_admin(years_experience: usize) -> &'static str {
    match years_experience {
        0..=2 => "APS 1-2",
        3..=5 => "APS 3-5",
        5..=8 => "APS 5-8",
        8..=10 => "EL1 8-10",
        10..=13 => "EL2 10-13",
        _ if years_experience >= 13 => "SES",
        _ => "Invalid Experience",
    }
}


fn match_academic(years_experience: usize) -> &'static str {
    match years_experience {
        3..=5 => "APS 3-5",
        5..=8 => "APS 5-8",
        8..=10 => "EL1 8-10",
        10..=13 => "EL2 10-13",
        _ if years_experience >= 13 => "SES",
        _ => "Invalid Experience",
    }
}


fn match_lawyer(years_experience: usize) -> &'static str {
    match years_experience {
        0..=2 => "APS 1-2",
        3..=5 => "APS 3-5",
        5..=8 => "APS 5-8",
        8..=10 => "EL1 8-10",
        10..=13 => "EL2 10-13",
        _ if years_experience >= 13 => "SES",
        _ => "Invalid Experience",
    }
}


fn match_teacher(years_experience: usize) -> &'static str {
    match years_experience {
        0..=2 => "APS 1-2",
        3..=5 => "APS 3-5",
        5..=8 => "APS 5-8",
        8..=10 => "EL1 8-10",
        10..=13 => "EL2 10-13",
        _ if years_experience >= 13 => "SES",
        _ => "Invalid Experience",
    }
}

