aws_region               = "us-east-1"
availability_zones       = ["us-east-1a", "us-east-1b"]
rds_username             = "development_user"
rds_password             = "development_password"
ecs_execution_role_arn   = "arn:aws:iam::123456789012:role/ecsExecutionRole"
app_image                = "your-ecr-uri/zpk-app:latest"
