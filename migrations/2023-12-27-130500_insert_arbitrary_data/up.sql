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
