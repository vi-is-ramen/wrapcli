#!/usr/bin/env bash

# --- Configuration ---
# 1. Create a file with your variables (e.g., 'template.env')
# 2. Fill it with placeholder definitions: PROJECT="my-awesome-app"
# 3. Run this script to replace placeholders in all project files

set -e  # Exit on any error

# --- Helper functions ---
show_help() {
    cat << EOF
Usage: $0 [OPTIONS]

Options:
  -e, --env-file FILE   Path to the environment file with variables (default: template.env)
  -d, --target-dir DIR  Target directory to process (default: current directory)
  -h, --help            Show this help message

Examples:
  $0                                    # Uses ./template.env and processes current directory
  $0 -e my-vars.env -d ./project        # Uses my-vars.env and processes ./project directory
EOF
}

# --- Argument parsing ---
ENV_FILE="expand.cfg"
TARGET_DIR="."

while [[ $# -gt 0 ]]; do
    case $1 in
        -e|--env-file)
            ENV_FILE="$2"
            shift 2
            ;;
        -d|--target-dir)
            TARGET_DIR="$2"
            shift 2
            ;;
        -h|--help)
            show_help
            exit 0
            ;;
        *)
            echo "Error: Unknown option $1"
            show_help
            exit 1
            ;;
    esac
done

# --- Main script ---
# Check if environment file exists
if [ ! -f "$ENV_FILE" ]; then
    echo "Error: Environment file '$ENV_FILE' not found!"
    exit 1
fi

# Check if target directory exists
if [ ! -d "$TARGET_DIR" ]; then
    echo "Error: Target directory '$TARGET_DIR' not found!"
    exit 1
fi

# Source the environment file to load variables
set -a
# shellcheck source=/dev/null
source "$ENV_FILE"
set +a

# Find all files except binary files and excluded directories
# You can adjust the find command as needed
echo "Processing files in '$TARGET_DIR'..."

# Use 'find' to get all files, then use 'while read' to process them
find "$TARGET_DIR" -type f \
    -not -path "*/\.*" \
    -not -name "expand.sh" \
    -not -name "$ENV_FILE" \
    -not -name "*.png" -not -name "*.jpg" -not -name "*.ico" \
    -not -name "*.gz" -not -name "*.zip" \
    -not -path "*/.git/*" \
    -print0 | while IFS= read -r -d '' file; do

    # Use sed to replace all @VARIABLE@ with their values
    # We build a sed script dynamically
    sed_script=""
    while IFS='=' read -r var value; do
        # Skip empty lines and comments
        [[ -z "$var" || "$var" =~ ^[[:space:]]*# ]] && continue
        # Remove leading/trailing whitespace
        var=$(echo "$var" | xargs)
        # Get the value of the variable from the environment
        value="${!var}"
        # Escape special characters for sed (/, &, etc.)
        value=$(printf '%s\n' "$value" | sed -e 's/[\/&]/\\&/g')
        # Add to sed script
        sed_script="$sed_script s/@$var@/$value/g;"
    done < "$ENV_FILE"

    # Apply the sed script to the file
    sed -i "$sed_script" "$file"
    echo "Processed: $file"
done

echo "Variable expansion complete!"
