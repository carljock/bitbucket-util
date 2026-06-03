#!/usr/bin/env python3
"""
OpenAPI Specification Patching Tool

Applies transformations to Bitbucket OpenAPI spec to work around
API inconsistencies before code generation.

Usage:
    python3 scripts/patch_openapi.py \\
        --input bitbucket-openapi-2.300.163.json \\
        --patches openapi-patches.yaml \\
        --output bitbucket-openapi-patched-2.300.163.json
"""

import json
import sys
import argparse
from pathlib import Path
from typing import Any, Dict, List
import yaml


class OpenAPIPatcher:
    """Applies patches to an OpenAPI specification."""

    def __init__(self, spec: Dict[str, Any]):
        self.spec = spec
        self.changes: List[str] = []

    def make_field_optional(self, schema_name: str, field_name: str) -> bool:
        """
        Remove a field from the 'required' array in a schema.

        Args:
            schema_name: Name of the schema to modify
            field_name: Name of the field to make optional

        Returns:
            True if the change was applied, False if field was already optional
            or schema not found
        """
        schemas = self.spec.get("components", {}).get("schemas", {})

        if schema_name not in schemas:
            self.changes.append(
                f"⚠️  Schema '{schema_name}' not found - skipping"
            )
            return False

        schema = schemas[schema_name]
        required = schema.get("required", [])

        if field_name not in required:
            self.changes.append(
                f"ℹ️  Field '{field_name}' already optional in '{schema_name}' - skipping"
            )
            return False

        # Remove the field from required array
        schema["required"] = [f for f in required if f != field_name]

        # If required array is now empty, remove it entirely
        if not schema["required"]:
            del schema["required"]

        self.changes.append(
            f"✓ Made field '{field_name}' optional in schema '{schema_name}'"
        )
        return True

    def add_property(
        self, schema_name: str, property_name: str, property_spec: Dict[str, Any]
    ) -> bool:
        """
        Add a new property to a schema.

        Args:
            schema_name: Name of the schema to modify
            property_name: Name of the property to add
            property_spec: Property specification (type, description, etc.)

        Returns:
            True if the property was added, False if it already exists
        """
        schemas = self.spec.get("components", {}).get("schemas", {})

        if schema_name not in schemas:
            self.changes.append(
                f"⚠️  Schema '{schema_name}' not found - skipping"
            )
            return False

        schema = schemas[schema_name]
        properties = schema.setdefault("properties", {})

        if property_name in properties:
            self.changes.append(
                f"ℹ️  Property '{property_name}' already exists in '{schema_name}' - skipping"
            )
            return False

        properties[property_name] = property_spec
        self.changes.append(
            f"✓ Added property '{property_name}' to schema '{schema_name}'"
        )
        return True

    def change_property_type(
        self, schema_name: str, property_name: str, new_type: str
    ) -> bool:
        """
        Change the type of a property in a schema.

        Args:
            schema_name: Name of the schema to modify
            property_name: Name of the property to modify
            new_type: New type for the property

        Returns:
            True if the type was changed, False if property not found
        """
        schemas = self.spec.get("components", {}).get("schemas", {})

        if schema_name not in schemas:
            self.changes.append(
                f"⚠️  Schema '{schema_name}' not found - skipping"
            )
            return False

        schema = schemas[schema_name]
        properties = schema.get("properties", {})

        if property_name not in properties:
            self.changes.append(
                f"⚠️  Property '{property_name}' not found in '{schema_name}' - skipping"
            )
            return False

        old_type = properties[property_name].get("type", "unknown")
        properties[property_name]["type"] = new_type
        self.changes.append(
            f"✓ Changed type of '{property_name}' in '{schema_name}' from '{old_type}' to '{new_type}'"
        )
        return True

    def set_nullable(self, schema_name: str, property_name: str) -> bool:
        """
        Set a property as nullable.

        Args:
            schema_name: Name of the schema to modify
            property_name: Name of the property to make nullable

        Returns:
            True if the property was made nullable, False otherwise
        """
        schemas = self.spec.get("components", {}).get("schemas", {})

        if schema_name not in schemas:
            self.changes.append(
                f"⚠️  Schema '{schema_name}' not found - skipping"
            )
            return False

        schema = schemas[schema_name]
        properties = schema.get("properties", {})

        if property_name not in properties:
            self.changes.append(
                f"⚠️  Property '{property_name}' not found in '{schema_name}' - skipping"
            )
            return False

        if properties[property_name].get("nullable") is True:
            self.changes.append(
                f"ℹ️  Property '{property_name}' already nullable in '{schema_name}' - skipping"
            )
            return False

        properties[property_name]["nullable"] = True
        self.changes.append(
            f"✓ Made property '{property_name}' nullable in schema '{schema_name}'"
        )
        return True

    def flatten_schema(
        self,
        schema_name: str,
        base_schema_name: str,
    ) -> bool:
        """
        Flatten a schema that uses allOf inheritance by merging properties.

        Converts:
            schema_a:
              allOf:
                - $ref: '#/components/schemas/schema_b'
                - properties: {...}

        To:
            schema_a:
              type: object
              properties:
                (all from schema_b + all from original schema_a)

        Args:
            schema_name: Name of the schema to flatten
            base_schema_name: Name of the base schema referenced in allOf

        Returns:
            True if the schema was flattened, False otherwise
        """
        schemas = self.spec.get("components", {}).get("schemas", {})

        if schema_name not in schemas:
            self.changes.append(
                f"⚠️  Schema '{schema_name}' not found - skipping"
            )
            return False

        if base_schema_name not in schemas:
            self.changes.append(
                f"⚠️  Base schema '{base_schema_name}' not found - skipping"
            )
            return False

        schema = schemas[schema_name]
        base_schema = schemas[base_schema_name]

        if "allOf" not in schema:
            self.changes.append(
                f"ℹ️  Schema '{schema_name}' does not use allOf - skipping"
            )
            return False

        # Extract all properties from base schema
        base_properties = base_schema.get("properties", {}).copy()

        # Extract properties from the inline schema in allOf
        all_of_items = schema.get("allOf", [])
        inline_properties = {}
        for item in all_of_items:
            if isinstance(item, dict) and "properties" in item:
                inline_properties.update(item.get("properties", {}))

        # Merge all properties
        merged_properties = {**base_properties, **inline_properties}

        # Replace the schema structure
        schema["type"] = "object"
        schema["properties"] = merged_properties
        schema["additionalProperties"] = True

        # Add a type property with a fixed value matching the schema name
        # Convert schema_name from snake_case to the enum value (e.g., pipeline_ref_target)
        if "type" not in schema.get("properties", {}):
            schema["properties"]["type"] = {
                "type": "string",
                "enum": [schema_name],
                "description": f"Type discriminator for {schema_name}"
            }

        # Keep discriminator if it exists in base
        if "discriminator" in base_schema and "discriminator" not in schema:
            schema["discriminator"] = base_schema["discriminator"]

        # Remove allOf
        del schema["allOf"]

        self.changes.append(
            f"✓ Flattened schema '{schema_name}' with {len(merged_properties)} properties"
        )
        return True

    def add_discriminator_mapping(
        self,
        schema_name: str,
        property_name: str,
        mapping: Dict[str, str],
    ) -> bool:
        """
        Add a discriminator mapping to a schema.

        For schemas using allOf that reference a base schema with discriminator,
        this adds a discriminator directly to the schema with the mapping.

        Args:
            schema_name: Name of the schema to modify
            property_name: Name of the discriminator property
            mapping: Dictionary mapping discriminator values to schema references

        Returns:
            True if the mapping was added, False otherwise
        """
        schemas = self.spec.get("components", {}).get("schemas", {})

        if schema_name not in schemas:
            self.changes.append(
                f"⚠️  Schema '{schema_name}' not found - skipping"
            )
            return False

        schema = schemas[schema_name]

        # If schema doesn't have discriminator directly, add it
        if "discriminator" not in schema:
            if "allOf" in schema:
                # For allOf schemas that inherit from object, add discriminator here
                schema["discriminator"] = {
                    "propertyName": property_name,
                    "mapping": mapping,
                }
                self.changes.append(
                    f"✓ Added discriminator mapping to allOf schema '{schema_name}' with {len(mapping)} entries"
                )
                return True
            else:
                self.changes.append(
                    f"⚠️  No discriminator found and not an allOf schema '{schema_name}' - skipping"
                )
                return False

        discriminator = schema["discriminator"]
        if discriminator.get("propertyName") != property_name:
            self.changes.append(
                f"⚠️  Discriminator property name mismatch in '{schema_name}' - skipping"
            )
            return False

        if "mapping" in discriminator and discriminator["mapping"]:
            self.changes.append(
                f"ℹ️  Discriminator mapping already exists in '{schema_name}' - skipping"
            )
            return False

        discriminator["mapping"] = mapping
        self.changes.append(
            f"✓ Added discriminator mapping to schema '{schema_name}' with {len(mapping)} entries"
        )
        return True

    def apply_patch(self, patch: Dict[str, Any]) -> bool:
        """
        Apply a single patch to the specification.

        Args:
            patch: Patch configuration dictionary

        Returns:
            True if patch was applied successfully, False otherwise
        """
        patch_type = patch.get("type")
        patch_name = patch.get("name", "Unnamed patch")

        if patch_type in ("make_field_optional", "remove_required_field"):
            return self.make_field_optional(
                patch["schema"], patch["field"]
            )
        elif patch_type == "add_property":
            return self.add_property(
                patch["schema"], patch["property"], patch["spec"]
            )
        elif patch_type == "change_type":
            return self.change_property_type(
                patch["schema"], patch["property"], patch["new_type"]
            )
        elif patch_type == "set_nullable":
            return self.set_nullable(patch["schema"], patch["property"])
        elif patch_type == "flatten_schema":
            return self.flatten_schema(
                patch["schema"],
                patch["baseSchema"],
            )
        elif patch_type == "add_discriminator_mapping":
            return self.add_discriminator_mapping(
                patch["schema"],
                patch["propertyName"],
                patch["mapping"],
            )
        else:
            self.changes.append(
                f"⚠️  Unknown patch type '{patch_type}' in '{patch_name}' - skipping"
            )
            return False

    def apply_patches(self, patches: List[Dict[str, Any]]) -> None:
        """
        Apply all patches from configuration.

        Args:
            patches: List of patch configuration dictionaries
        """
        for i, patch in enumerate(patches, 1):
            name = patch.get("name", f"Patch #{i}")
            reason = patch.get("reason", "No reason provided")

            print(f"\n[{i}/{len(patches)}] {name}")
            print(f"    Reason: {reason.strip()[:80]}...")

            self.apply_patch(patch)

    def get_changes_log(self) -> List[str]:
        """Return list of changes made."""
        return self.changes

    def get_spec(self) -> Dict[str, Any]:
        """Return the modified specification."""
        return self.spec


def load_openapi_spec(path: Path) -> Dict[str, Any]:
    """Load OpenAPI specification from JSON file."""
    with open(path, "r") as f:
        return json.load(f)


def load_patches_config(path: Path) -> List[Dict[str, Any]]:
    """Load patches configuration from YAML file."""
    with open(path, "r") as f:
        config = yaml.safe_load(f)
        return config.get("patches", [])


def save_openapi_spec(spec: Dict[str, Any], path: Path) -> None:
    """Save OpenAPI specification to JSON file."""
    with open(path, "w") as f:
        json.dump(spec, f, indent=2)


def main():
    parser = argparse.ArgumentParser(
        description="Patch OpenAPI specification to work around API inconsistencies"
    )
    parser.add_argument(
        "--input",
        type=Path,
        required=True,
        help="Input OpenAPI specification JSON file",
    )
    parser.add_argument(
        "--patches",
        type=Path,
        required=True,
        help="Patches configuration YAML file",
    )
    parser.add_argument(
        "--output",
        type=Path,
        required=True,
        help="Output patched OpenAPI specification JSON file",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Show what would be changed without writing output",
    )

    args = parser.parse_args()

    # Validate input files exist
    if not args.input.exists():
        print(f"Error: Input file not found: {args.input}", file=sys.stderr)
        sys.exit(1)

    if not args.patches.exists():
        print(f"Error: Patches file not found: {args.patches}", file=sys.stderr)
        sys.exit(1)

    # Load files
    print(f"Loading OpenAPI spec from: {args.input}")
    spec = load_openapi_spec(args.input)

    print(f"Loading patches from: {args.patches}")
    patches = load_patches_config(args.patches)

    if not patches:
        print("No patches to apply.")
        sys.exit(0)

    # Apply patches
    print(f"\nApplying {len(patches)} patch(es)...")
    patcher = OpenAPIPatcher(spec)
    patcher.apply_patches(patches)

    # Print summary
    print("\n" + "=" * 60)
    print("SUMMARY OF CHANGES")
    print("=" * 60)
    for change in patcher.get_changes_log():
        print(change)

    # Save output
    if args.dry_run:
        print("\nDry run - no output file written.")
    else:
        print(f"\nSaving patched specification to: {args.output}")
        save_openapi_spec(patcher.get_spec(), args.output)
        print("Done!")


if __name__ == "__main__":
    main()
