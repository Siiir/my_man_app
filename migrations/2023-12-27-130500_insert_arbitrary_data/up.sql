-- Consts

INSERT INTO MIN_SALARY VALUES (10000.00);

-- Normal data

-- Insert data into table Human
INSERT INTO Human (id, name, surname, nickname) VALUES (1, 'John', 'Doe', 'johndoe');
INSERT INTO Human (id, name, surname, nickname) VALUES (2, 'Jane', 'Doe', 'janedoe');
INSERT INTO Human (id, name, surname, nickname) VALUES (3, 'Bob', 'Smith', 'bobsmith');
INSERT INTO Human (id, name, surname, nickname) VALUES (4, 'Alice', 'Johnson', 'alicejohnson');
INSERT INTO Human (id, name, surname, nickname) VALUES (5, 'Mike', 'Brown', 'mikebrown');

-- Insert data into table Client
INSERT INTO Client (human_id) VALUES (1);
INSERT INTO Client (human_id) VALUES (2);
INSERT INTO Client (human_id) VALUES (3);
INSERT INTO Client (human_id) VALUES (4);
INSERT INTO Client (human_id) VALUES (5);

-- Insert data into table Employee
INSERT INTO Employee (human_id, `rank`, salary) VALUES (1, 'Manager', 50000.00);
INSERT INTO Employee (human_id, `rank`, salary) VALUES (2, 'Analyst', 30000.00);
INSERT INTO Employee (human_id, `rank`, salary) VALUES (3, 'Developer', 40000.00);
INSERT INTO Employee (human_id, `rank`, salary) VALUES (4, 'Consultant', 60000.00);
INSERT INTO Employee (human_id, `rank`, salary) VALUES (5, 'Engineer', 45000.00);

-- Insert data into table Project
INSERT INTO Project (id, fancy_name, start_date, end_date, deadline, responsible) VALUES (1, 'Project Alpha', '2022-01-01', '2022-03-01', '2022-03-01 00:00:00', 1);
INSERT INTO Project (id, fancy_name, start_date, end_date, deadline, responsible) VALUES (2, 'Project Beta', '2022-02-01', NULL, '2022-06-01 00:00:00', 2);
INSERT INTO Project (id, fancy_name, start_date, end_date, deadline, responsible) VALUES (3, 'Project Gamma', '2022-04-01', '2022-07-01', '2022-07-01 00:00:00', 3);
INSERT INTO Project (id, fancy_name, start_date, end_date, deadline, responsible) VALUES (4, 'Project Delta', '2022-05-01', NULL, '2022-09-01 00:00:00', 4);
INSERT INTO Project (id, fancy_name, start_date, end_date, deadline, responsible) VALUES (5, 'Project Epsilon', '2022-08-01', '2022-10-01', '2022-10-01 00:00:00', 5);

-- Insert data into table Client_Project
INSERT INTO Client_Project (project_id, client_id, pays, spectrum_of_interest) VALUES (1, 1, 10000.00, 'Marketing');
INSERT INTO Client_Project (project_id, client_id, pays, spectrum_of_interest) VALUES (2, 2, 20000.00, 'Finance');
INSERT INTO Client_Project (project_id, client_id, pays, spectrum_of_interest) VALUES (3, 3, 30000.00, 'IT');
INSERT INTO Client_Project (project_id, client_id, pays, spectrum_of_interest) VALUES (4, 4, 40000.00, 'Sales');
INSERT INTO Client_Project (project_id, client_id, pays, spectrum_of_interest) VALUES (5, 5, 50000.00, 'Research');

-- Insert data into table Skills
INSERT INTO Skills (name, priority) VALUES ('Leadership', 1);
INSERT INTO Skills (name, priority) VALUES ('Data Analysis', 2);
INSERT INTO Skills (name, priority) VALUES ('Software Development', 3);
INSERT INTO Skills (name, priority) VALUES ('Consulting', 4);
INSERT INTO Skills (name, priority) VALUES ('Problem Solving', 5);

-- Insert data into table FriendCompany
INSERT INTO FriendCompany (id, abbr_name, full_name) VALUES (1, 'ABC', 'ABC Inc.');
INSERT INTO FriendCompany (id, abbr_name, full_name) VALUES (2, 'DEF', 'DEF Corp.');
INSERT INTO FriendCompany (id, abbr_name, full_name) VALUES (3, 'GHI', 'GHI Ltd.');
INSERT INTO FriendCompany (id, abbr_name, full_name) VALUES (4, 'JKL', 'JKL Co.');
INSERT INTO FriendCompany (id, abbr_name, full_name) VALUES (5, 'MNO', 'MNO LLC');

-- Insert data into table Project_Employee
INSERT INTO Project_Employee (employee_human_id, project_id, function_in_project) VALUES (1, 1, 'Project Manager');
INSERT INTO Project_Employee (employee_human_id, project_id, function_in_project) VALUES (2, 1, 'Data Analyst');
INSERT INTO Project_Employee (employee_human_id, project_id, function_in_project) VALUES (3, 2, 'Software Developer');
INSERT INTO Project_Employee (employee_human_id, project_id, function_in_project) VALUES (4, 3, 'Consultant');
INSERT INTO Project_Employee (employee_human_id, project_id, function_in_project) VALUES (5, 4, 'Engineer');

-- Insert data into table Outsourcing
INSERT INTO Outsourcing (task, cost, project_id, friend_company_id) VALUES ('Web design', 10000.00, 1, 1);
INSERT INTO Outsourcing (task, cost, project_id, friend_company_id) VALUES ('Market research', 20000.00, 2, 2);
INSERT INTO Outsourcing (task, cost, project_id, friend_company_id) VALUES ('Software testing', 15000.00, 3, 3);
INSERT INTO Outsourcing (task, cost, project_id, friend_company_id) VALUES ('Sales training', 25000.00, 4, 4);
INSERT INTO Outsourcing (task, cost, project_id, friend_company_id) VALUES ('Data analysis', 30000.00, 5, 5);

-- Insert data into table Employee_Skill
INSERT INTO Employee_Skill (employee_human_id, skills_name) VALUES (1, 'Leadership');
INSERT INTO Employee_Skill (employee_human_id, skills_name) VALUES (1, 'Data Analysis');
INSERT INTO Employee_Skill (employee_human_id, skills_name) VALUES (2, 'Data Analysis');
INSERT INTO Employee_Skill (employee_human_id, skills_name) VALUES (2, 'Problem Solving');
INSERT INTO Employee_Skill (employee_human_id, skills_name) VALUES (3, 'Software Development');
INSERT INTO Employee_Skill (employee_human_id, skills_name) VALUES (3, 'Data Analysis');
INSERT INTO Employee_Skill (employee_human_id, skills_name) VALUES (4, 'Consulting');
INSERT INTO Employee_Skill (employee_human_id, skills_name) VALUES (4, 'Problem Solving');
INSERT INTO Employee_Skill (employee_human_id, skills_name) VALUES (5, 'Software Development');
INSERT INTO Employee_Skill (employee_human_id, skills_name) VALUES (5, 'Problem Solving');

-- Insert PJATK ppl
INSERT INTO Human (id, name, surname, nickname) VALUES (6, 'Abderrahmene', 'BOUTCHICHA', 's22878');
INSERT INTO Human (id, name, surname, nickname) VALUES (7, 'Abdujabbor', 'Makhmudov', 's27342');
INSERT INTO Human (id, name, surname, nickname) VALUES (8, 'Abdulelah', 'Almansour', 's23407');
INSERT INTO Human (id, name, surname, nickname) VALUES (9, 'ABDULKADIR', 'YAGLI', 's8079');
INSERT INTO Human (id, name, surname, nickname) VALUES (10, 'ABDULKHAKIM', 'KHAKIMOV', 's28718');
INSERT INTO Human (id, name, surname, nickname) VALUES (11, 'Abdulla', 'Mohamed', 's22816');
INSERT INTO Human (id, name, surname, nickname) VALUES (12, 'Abdulla', 'Mohamed', 'abdulm');
INSERT INTO Human (id, name, surname, nickname) VALUES (13, 'ABDULLAH', 'BEÇET', 's18377');
INSERT INTO Human (id, name, surname, nickname) VALUES (14, 'Abdullakh', 'Rustamov', 's32445');
INSERT INTO Human (id, name, surname, nickname) VALUES (15, 'Abdulqasem', 'Bakhshi', 's29148');
INSERT INTO Human (id, name, surname, nickname) VALUES (16, 'Abdulrahman', 'Abo', 's22250');
INSERT INTO Human (id, name, surname, nickname) VALUES (17, 'ABDULSAMED', 'SAY', 's18466');
INSERT INTO Human (id, name, surname, nickname) VALUES (18, 'Abdulwahab', 'Alhajmohammad', 's19244');
INSERT INTO Human (id, name, surname, nickname) VALUES (19, 'Abdumalik', 'Abdusalomov', 's21959');
INSERT INTO Human (id, name, surname, nickname) VALUES (20, 'ABDURAKHMON', 'YUNUSOV', 's19505');
INSERT INTO Human (id, name, surname, nickname) VALUES (21, 'Abel', 'Hazi', 's20641');
INSERT INTO Human (id, name, surname, nickname) VALUES (22, 'Abhishek', 'Kumar', 's28357');
INSERT INTO Human (id, name, surname, nickname) VALUES (23, 'Abhishek', 'Nalwad', 'so0283');
INSERT INTO Human (id, name, surname, nickname) VALUES (24, 'Abhrajeet', 'Mukherjee', 'so0296');
INSERT INTO Human (id, name, surname, nickname) VALUES (25, 'Abigail', 'Rybinska', 'so0257');
INSERT INTO Human (id, name, surname, nickname) VALUES (26, 'Abror', 'Kamalov', 's10527');
INSERT INTO Human (id, name, surname, nickname) VALUES (27, 'Abrorbek', 'Turgunboev', 's23189');
INSERT INTO Human (id, name, surname, nickname) VALUES (28, 'Absolwenci', 'Liceum', 's19139');
INSERT INTO Human (id, name, surname, nickname) VALUES (29, 'Abusat', 'Aghali', null);
INSERT INTO Human (id, name, surname, nickname) VALUES (30, 'ABZAL', 'ZHAKSYLYK', 's21460');
INSERT INTO Human (id, name, surname, nickname) VALUES (31, 'Achut', 'Poudel', 's24202');
INSERT INTO Human (id, name, surname, nickname) VALUES (32, 'Ada', 'Bajurko', 's4941');
INSERT INTO Human (id, name, surname, nickname) VALUES (33, 'Ada', 'Baradziej', 's10378');
INSERT INTO Human (id, name, surname, nickname) VALUES (34, 'Ada', 'Biedka', 's31075');
INSERT INTO Human (id, name, surname, nickname) VALUES (35, 'Ada', 'Cendrowska', 'adabiedka');
INSERT INTO Human (id, name, surname, nickname) VALUES (36, 'Ada', 'Ciuba', 's5209');
INSERT INTO Human (id, name, surname, nickname) VALUES (37, 'Ada', 'Dziurzyńska', 's8699');
INSERT INTO Human (id, name, surname, nickname) VALUES (38, 'Ada', 'Emilia', 's26440');
INSERT INTO Human (id, name, surname, nickname) VALUES (39, 'Ada', 'Göbel', 's3984');
INSERT INTO Human (id, name, surname, nickname) VALUES (40, 'Ada', 'Guzniczak', 's25454');
INSERT INTO Human (id, name, surname, nickname) VALUES (41, 'Ada', 'Iteji', 'adaguzniczak');
INSERT INTO Human (id, name, surname, nickname) VALUES (42, 'Ada', 'Jedlinska', 'k350');
INSERT INTO Human (id, name, surname, nickname) VALUES (43, 'Ada', 'Jedlińska', 'pd1188');
INSERT INTO Human (id, name, surname, nickname) VALUES (44, 'Ada', 'Karaszewska', 'ada');
INSERT INTO Human (id, name, surname, nickname) VALUES (45, 'Ada', 'Lewicka', 's12194');
INSERT INTO Human (id, name, surname, nickname) VALUES (46, 'Ada', 'Lubinska', 's8795');
INSERT INTO Human (id, name, surname, nickname) VALUES (47, 'Ada', 'Miecznikowska', 's13290');
INSERT INTO Human (id, name, surname, nickname) VALUES (48, 'Ada', 'Mrozinska', 's13968');
INSERT INTO Human (id, name, surname, nickname) VALUES (49, 'Ada', 'Nowacka', 's8954');
INSERT INTO Human (id, name, surname, nickname) VALUES (50, 'Ada', 'Pawelec', 's11478');
INSERT INTO Human (id, name, surname, nickname) VALUES (51, 'Ada', 'Skwira', 's26012');
INSERT INTO Human (id, name, surname, nickname) VALUES (52, 'Ada', 'Sobczyk', 's9122');
INSERT INTO Human (id, name, surname, nickname) VALUES (53, 'Ada', 'Sulewska', 's25302');
INSERT INTO Human (id, name, surname, nickname) VALUES (54, 'Ada', 'Szczepkowska', 's10367');
INSERT INTO Human (id, name, surname, nickname) VALUES (55, 'Ada', 'Wiśniewska', 's27538');
INSERT INTO Human (id, name, surname, nickname) VALUES (56, 'Adaeze', 'Udeh', 's29477');
INSERT INTO Human (id, name, surname, nickname) VALUES (57, 'Adam', 'Adach', 's25856');
INSERT INTO Human (id, name, surname, nickname) VALUES (58, 'Adam', 'Aleksandrowicz', 's28760');
INSERT INTO Human (id, name, surname, nickname) VALUES (59, 'Adam', 'Aleksandrowicz', 's24732');
INSERT INTO Human (id, name, surname, nickname) VALUES (60, 'Adam', 'Andrzej', 'l0441');
INSERT INTO Human (id, name, surname, nickname) VALUES (61, 'Adam', 'Andrzej', 's4543');
INSERT INTO Human (id, name, surname, nickname) VALUES (62, 'Adam', 'Arkadiusz', 's3086');
INSERT INTO Human (id, name, surname, nickname) VALUES (63, 'Adam', 'Aszyk', 's3348');
INSERT INTO Human (id, name, surname, nickname) VALUES (64, 'Adam', 'Augustynowicz', 's18198');
INSERT INTO Human (id, name, surname, nickname) VALUES (65, 'Adam', 'Bachanek', 's8406');
INSERT INTO Human (id, name, surname, nickname) VALUES (66, 'Adam', 'Baginski', 'so0227');
-- Special PJATK ppl
INSERT INTO Human (id, name, surname, nickname) VALUES (67, 'Szymon', 'Pierzchała', 'Pieszy');
INSERT INTO Human (id, name, surname, nickname) VALUES (68, 'Michał', 'Tomaszewski', 'Tomaszew');
