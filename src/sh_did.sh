did=$(dfx canister call my_example __get_candid_interface_tmp_hack)
did=${did#*"\""}
did=${did%"\""*}
echo "$did" > canisters/my_example/src/my_example.did 
echo "$did"
echo "==================my_example"