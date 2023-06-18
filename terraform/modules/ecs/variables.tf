variable "app_image" {
  type        = string
  description = "Docker image for the zkp app"
}

variable "ecs_execution_role_arn" {
  type        = string
  description = "Execution role ARN for ECS"
}

variable "security_group_id" {
  type        = string
  description = "Security Group ID"
}

variable "subnets" {
  type        = list(string)
  description = "Subnets for the ECS service"
}

variable "database_uri" {
  type        = string
  description = "Database URI for the application"
}
