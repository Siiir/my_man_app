// @generated automatically by Diesel CLI.

diesel::table! {
    Client (human_id) {
        human_id -> Integer,
    }
}

diesel::table! {
    Client_Project (id) {
        id -> Integer,
        project_id -> Integer,
        client_id -> Integer,
        pays -> Decimal,
        #[max_length = 800]
        spectrum_of_interest -> Varchar,
    }
}

diesel::table! {
    Employee (human_id) {
        human_id -> Integer,
        #[max_length = 12]
        rank -> Varchar,
        salary -> Decimal,
    }
}

diesel::table! {
    Employee_Skill (id) {
        id -> Integer,
        employee_human_id -> Integer,
        #[max_length = 80]
        skills_name -> Varchar,
    }
}

diesel::table! {
    FriendCompany (id) {
        id -> Integer,
        #[max_length = 10]
        abbr_name -> Nullable<Varchar>,
        #[max_length = 120]
        full_name -> Varchar,
    }
}

diesel::table! {
    Human (id) {
        id -> Integer,
        #[max_length = 80]
        name -> Varchar,
        #[max_length = 80]
        surname -> Varchar,
        #[max_length = 16]
        nickname -> Nullable<Varchar>,
    }
}

diesel::table! {
    MIN_SALARY (val) {
        val -> Decimal,
    }
}

diesel::table! {
    Outsourcing (id) {
        id -> Integer,
        #[max_length = 1000]
        task -> Varchar,
        cost -> Decimal,
        project_id -> Integer,
        friend_company_id -> Integer,
    }
}

diesel::table! {
    Project (id) {
        id -> Integer,
        #[max_length = 20]
        fancy_name -> Varchar,
        start_date -> Date,
        end_date -> Nullable<Date>,
        deadline -> Timestamp,
        responsible -> Nullable<Integer>,
    }
}

diesel::table! {
    Project_Employee (id) {
        id -> Integer,
        employee_human_id -> Integer,
        project_id -> Integer,
        #[max_length = 80]
        function_in_project -> Varchar,
    }
}

diesel::table! {
    Skills (name) {
        #[max_length = 80]
        name -> Varchar,
        priority -> Smallint,
    }
}

diesel::joinable!(Client -> Human (human_id));
diesel::joinable!(Client_Project -> Client (client_id));
diesel::joinable!(Client_Project -> Project (project_id));
diesel::joinable!(Employee -> Human (human_id));
diesel::joinable!(Employee_Skill -> Employee (employee_human_id));
diesel::joinable!(Employee_Skill -> Skills (skills_name));
diesel::joinable!(Outsourcing -> FriendCompany (friend_company_id));
diesel::joinable!(Outsourcing -> Project (project_id));
diesel::joinable!(Project -> Employee (responsible));
diesel::joinable!(Project_Employee -> Employee (employee_human_id));
diesel::joinable!(Project_Employee -> Project (project_id));

diesel::allow_tables_to_appear_in_same_query!(
    Client,
    Client_Project,
    Employee,
    Employee_Skill,
    FriendCompany,
    Human,
    MIN_SALARY,
    Outsourcing,
    Project,
    Project_Employee,
    Skills,
);
