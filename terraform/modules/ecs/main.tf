resource "aws_ecs_cluster" "main" {
  name = "zkp-app-cluster"
}

resource "aws_ecs_task_definition" "main" {
  family                   = "zkp-app"
  network_mode             = "awsvpc"
  requires_compatibilities = ["FARGATE"]
  cpu                      = "256"
  memory                   = "512"
  execution_role_arn       = var.ecs_execution_role_arn

  container_definitions = jsonencode([{
    name      = "zkp-app-container"
    image     = var.app_image
    cpu       = 256
    memory    = 512
    essential = true
    portMappings = [{
      containerPort = 50051
      hostPort      = 50051
    }]
    environment = [{
      name  = "DATABASE_URI"
      value = var.database_uri
    }]
  }])
}

resource "aws_ecs_service" "main" {
  name            = "zkp-app-service"
  cluster         = aws_ecs_cluster.main.id
  task_definition = aws_ecs_task_definition.main.arn
  desired_count   = 1
  launch_type     = "FARGATE"

  network_configuration {
    subnets          = var.subnets
    security_groups  = [var.security_group_id]
    assign_public_ip = true
  }
}
