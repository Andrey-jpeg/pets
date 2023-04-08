CREATE TABLE IF NOT EXISTS Cat (
    id int PRIMARY KEY NOT NULL AUTO_INCREMENT,
    PetName varchar(255) NOT NULL,
    Color varchar(255),
    Status varchar(2555)
);
CREATE TABLE IF NOT EXISTS User (
    id int PRIMARY KEY NOT NULL AUTO_INCREMENT,
    FirstName varchar(255) NOT NULL,
    LastName varchar(255) NOT NULL,
    Username varchar(255) NOT NULL UNIQUE,
    Password varchar(255) NOT NULL,
    PhoneNumber varchar(20),
    email varchar(255)
);
CREATE TABLE IF NOT EXISTS UserPet (
	CatID INT,
    UserID INT,
    FOREIGN KEY (CatID) REFERENCES Cat(id),
    FOREIGN KEY (UserID) REFERENCES User(id),
    PRIMARY KEY(CatID, UserID)
);
CREATE TABLE IF NOT EXISTS PetLocation (
    CatID INT,
    Latitude float not null,
    Longitude float not null,
    PRIMARY KEY (CatID),
    FOREIGN KEY (CatID) REFERENCES Cat(id)
);

INSERT INTO Cat(PetName, Color, Status)
VALUES ("Cookie", "Sort og hvid", "Active");

INSERT INTO User(FirstName, LastName, Username, Password)
VALUES ("Hans", "Hansen", "H4N5", "5N4H");

INSERT INTO UserPet(CatID, UserID)
VALUES (1, 1);