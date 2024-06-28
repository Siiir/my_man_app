-- Consts

CREATE TABLE MIN_SALARY (
    val decimal(12,2),
    PRIMARY KEY (val) -- So that diesel will not complain.
);

-- Normal tables

CREATE TABLE Client
(
    human_id int NOT NULL AUTO_INCREMENT,
    PRIMARY KEY (human_id)
);

CREATE TABLE Client_Project
(
    id                  int            NOT NULL AUTO_INCREMENT,
    project_id          int            NOT NULL,
    client_id     int            NOT NULL,
    pays                decimal(12, 2) NOT NULL,
    spectrum_of_interest varchar(800)   NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE Employee
(
    human_id int            NOT NULL AUTO_INCREMENT,
    `rank`     varchar(12)    NOT NULL,
    salary   decimal(12, 2) NOT NULL,
    PRIMARY KEY (human_id)
);

CREATE TABLE Employee_Skill
(
    id                int         NOT NULL AUTO_INCREMENT,
    employee_human_id int         NOT NULL,
    skills_name       varchar(80) NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE FriendCompany
(
    id        int          NOT NULL AUTO_INCREMENT,
    abbr_name varchar(10),
    full_name varchar(120) NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE Human
(
    id        int         NOT NULL AUTO_INCREMENT,
    name      varchar(80) NOT NULL,
    surname   varchar(80) NOT NULL,
    nickname  varchar(16),
    PRIMARY KEY (id)
);

CREATE TABLE Outsourcing
(
    id                int            NOT NULL AUTO_INCREMENT,
    task              varchar(1000)  NOT NULL,
    cost              decimal(20, 2) NOT NULL,
    project_id        int            NOT NULL,
    friend_company_id int            NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE Project
(
    id           int         NOT NULL AUTO_INCREMENT,
    fancy_name   varchar(20) NOT NULL,
    start_date   date        NOT NULL,
    end_date     date,
    deadline     timestamp,
    responsible  int,
    PRIMARY KEY (id)
);

CREATE TABLE Project_Employee
(
    id                  int         NOT NULL AUTO_INCREMENT,
    employee_human_id   int         NOT NULL,
    project_id          int         NOT NULL,
    function_in_project varchar(80) NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE Skills
(
    name      varchar(80) NOT NULL,
    priority  smallint    NOT NULL,
    PRIMARY KEY (name)
);


-- foreign keys

ALTER TABLE Client_Project
    ADD CONSTRAINT Client_Project_refTo_Client
        FOREIGN KEY (client_id)
            REFERENCES Client (human_id);

ALTER TABLE Client_Project
    ADD CONSTRAINT Client_Project_refTo_Project
        FOREIGN KEY (project_id)
            REFERENCES Project (id);

ALTER TABLE Client
    ADD CONSTRAINT Client_refTo_Human
        FOREIGN KEY (human_id)
            REFERENCES Human (id);

ALTER TABLE Employee
    ADD CONSTRAINT Employee_refTo_Human
        FOREIGN KEY (human_id)
            REFERENCES Human (id);

ALTER TABLE Employee_Skill
    ADD CONSTRAINT Employee_Skill_refTo_Human
        FOREIGN KEY (employee_human_id)
            REFERENCES Employee (human_id);

ALTER TABLE Employee_Skill
    ADD CONSTRAINT Employee_Skill_refTo_Skill
        FOREIGN KEY (skills_name)
            REFERENCES Skills (name);

ALTER TABLE Outsourcing
    ADD CONSTRAINT Outsourcing_refTo_FriendCompany
        FOREIGN KEY (friend_company_id)
            REFERENCES FriendCompany (id);

ALTER TABLE Outsourcing
    ADD CONSTRAINT Outsourcing_refTo_Project
        FOREIGN KEY (project_id)
            REFERENCES Project (id);

ALTER TABLE Project
    ADD CONSTRAINT Project_refTo_Responsible_Human
        FOREIGN KEY (responsible)
            REFERENCES Employee (human_id);

ALTER TABLE Project_Employee
    ADD CONSTRAINT Project_Employee_refTo_Employee
        FOREIGN KEY (employee_human_id)
            REFERENCES Employee (human_id);

ALTER TABLE Project_Employee
    ADD CONSTRAINT Project_Employee_refTo_Project
        FOREIGN KEY (project_id)
            REFERENCES Project (id);
