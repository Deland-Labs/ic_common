use candid::Principal;
use rstest::*;

use ic_test_common::ic_api::init_test;

use super::*;

#[fixture]
fn service() -> UserService {
    UserService::new()
}


mod add_user {
    use super::*;

    #[rstest]
    fn test_add_user_success(_init_test: (),
                             service: UserService) {
        let id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
        let test_name = "test_name";
        let test_email = "test_email";

        // act
        let result = service.add_user(&id, test_name, test_email);

        // assert
        assert!(result.is_ok());
        USERS.with(|users| {
            let users = users.borrow();
            assert_eq!(users.len(), 1);
            let user = users.get(&id).unwrap();
            assert_eq!(user.get_name(), test_name);
            assert_eq!(user.get_email(), test_email);
        });
    }

    #[rstest]
    fn test_add_user_already_exists(_init_test: (),
                                    service: UserService) {
        let id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
        let test_name = "test_name";
        let test_email = "test_email";

        // act
        let result = service.add_user(&id, test_name, test_email);

        // assert
        assert!(result.is_ok());
        let result = service.add_user(&id, test_name, test_email);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), UserServiceError::UserAlreadyExists);
    }
}

mod get_by_id {
    use super::*;

    #[rstest]
    fn test_get_by_id_success(_init_test: (),
                              service: UserService) {
        let id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
        let test_name = "test_name";
        let test_email = "test_email";
        USERS.with(|users| {
            let mut users = users.borrow_mut();
            users.insert(id, User::new(id, test_name.to_string(), test_email.to_string()));
        });

        // act
        let result = service.get_user_by_id(&id);

        // assert
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.get_name(), test_name);
        assert_eq!(user.get_email(), test_email);
    }

    #[rstest]
    fn test_get_by_id_not_found(_init_test: (),
                                service: UserService) {
        let id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();

        // act
        let result = service.get_user_by_id(&id);

        // assert
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), UserServiceError::UserNotFound);
    }
}