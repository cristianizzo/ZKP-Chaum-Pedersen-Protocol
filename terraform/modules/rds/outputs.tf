output "endpoint" {
  value       = aws_db_instance.main.endpoint
  description = "The connection endpoint for the RDS instance"
}
