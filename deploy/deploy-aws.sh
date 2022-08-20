#!/bin/bash

usage() {
    echo "Usage: $0 [OPTION]..."
    echo "  -h, --help              Display this help and exit"
    echo "  -s, --stage             Specify deployment stage [stage, prod] (default: stage)"
    echo "  -p, --profile           Specify AWS profile"
}

parse_args() {
  stage="stage"
  aws_profile="default"

  while [ $# -gt 0 ]; do
    case $1 in
      -h|--help)
        usage
        exit 0
        ;;
      -s|--stage)
        stage=$2
        shift
        ;;
      -p|--profile)
        aws_profile=$2
        shift
        ;;
      *)
        echo "Unknown argument: $1"
        usage
        exit 1
        ;;
    esac
    shift
  done
}

main() {
  parse_args "$@"
  
  echo "Deploying to stage: $stage"
  echo "Using profile: $aws_profile"

  cargo lambda build --release

  sam deploy \
    --profile $aws_profile \
    --config-file ./deploy/$stage.toml
}

main "$@"
