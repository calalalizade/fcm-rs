# Change Log


## [0.2.0] - 2024-07-20

### Breaking Changes
- **Removed** the `project_id` parameter from the `FcmClient::new` function. The `project_id` is now extracted directly from the service account key JSON file.
  - **Migration:** Users must now call `FcmClient::new(service_account_key_path)` instead of `FcmClient::new(service_account_key_path, project_id)`.


### Enhancements
- **Added** improved error handling for FCM error responses.
- **Refactored** the `FcmClient` initialization process to simplify the creation of client instances.

### Caveats
- **Testing Status:** The "FCM success response" path has not been fully tested due to the absence of a mobile client app. Testing for valid device tokens is pending. Users are encouraged to verify this functionality with their mobile clients once available.

