current_identity=$(dfx identity get-principal)
my_example_id=$(dfx canister id my_example)
dfx canister call my_example add_user '(principal "rrkah-fqaaa-aaaaa-aaaaq-cai", "test_user", "test_email")'
dfx canister call my_example get_user_by_id '(principal "rrkah-fqaaa-aaaaa-aaaaq-cai")'


curl -L "http://127.0.0.1:8000/?canisterId=$my_example_id&id=rrkah-fqaaa-aaaaa-aaaaq-cai"
