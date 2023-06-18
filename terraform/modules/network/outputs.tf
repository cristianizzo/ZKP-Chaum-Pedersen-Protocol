output "vpc_id" {
  value       = aws_vpc.main.id
  description = "The ID of the VPC"
}

output "subnet_ids" {
  value       = aws_subnet.main.*.id
  description = "The IDs of the subnets"
}

output "ecs_security_group_id" {
  value       = aws_security_group.ecs.id
  description = "The ID of the ECS security group"
}

output "rds_security_group_id" {
  value       = aws_security_group.rds.id
  description = "The ID of the RDS security group"
}
