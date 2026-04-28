#!/usr/bin/env python3
"""Regenerate BENTERM banner + GitHub social preview via KIE.ai gpt-image-2.

Reads KIE_AI_API_KEY from E:/Rankenstein/Benjamin/.env (or KIE_AI_API_KEY env).
Submits two text-to-image tasks, polls for completion, downloads, and writes
the cropped/converted assets to E:/BENTERM/assets/.

Usage: python3 scripts/regenerate-banner.py [--banner-only|--social-only]
"""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
import time
import urllib.request
from pathlib import Path

KIE_CREATE_URL = "https://api.kie.ai/api/v1/jobs/createTask"
KIE_RECORD_URL = "https://api.kie.ai/api/v1/jobs/recordInfo"
MODEL = "gpt-image-2-text-to-image"
POLL_SECONDS = 5
TIMEOUT_SECONDS = 600
USER_AGENT = "benterm-rebrand/1.0"

REPO_ROOT = Path("E:/BENTERM").resolve()
ASSETS = REPO_ROOT / "assets"
TMP = REPO_ROOT / "tmp" / "rebrand"

BANNER_PROMPT = (
    'A polished GitHub README banner, ultra-wide 21:9 aspect ratio, against a deep '
    'pure-black background. On the left third: the wordmark "BENTERM" in big bold '
    'modern sans-serif, clean white, all caps, generous letter-spacing. Directly '
    'below the wordmark in smaller monospace lowercase: "attention-friendly '
    'terminal distro" with a single solid green block as the trailing cursor. On '
    'the right side: a stylized retro 1980s CRT television set, matte black housing '
    'with subtle metallic edges, sitting on a glossy reflective dark floor that '
    'mirrors the TV. Concentric rings of soft green phosphor glow radiate outward '
    'from the TV into the dark background like a sound-wave halo. The CRT screen '
    'shows green-on-black phosphor terminal text reading "benjamin@benterm ~" with '
    'a blinking green block cursor on the line below. On the upper right corner of '
    'the TV bezel, a small glowing red rectangular pixel-art badge reading "BEN" in '
    'all caps, slightly inset, with a faint red bloom. A tiny red power LED on the '
    'lower right of the bezel. Subtle scanline texture on the screen. Cinematic '
    'product render, deep contrast, no clutter, no extra text or logos. Captured '
    'with Sony A7R IV at 50mm f/2.8, soft directional rim light from the right, '
    'Wallpaper* magazine product editorial.'
)

SOCIAL_PROMPT = (
    'A premium GitHub social-preview card, 2:1 aspect ratio, against a deep '
    'pure-black background. Centered composition: the wordmark "BENTERM" in big '
    'bold modern sans-serif, clean white, all caps, dominating the upper half. '
    'Below the wordmark in smaller monospace lowercase: "attention-friendly '
    'terminal distro" with a single solid green block as the trailing cursor. '
    'Behind and slightly to the right, a stylized retro 1980s CRT television set, '
    'matte black housing with subtle metallic edges, sitting on a glossy '
    'reflective dark floor. Concentric rings of soft green phosphor glow radiate '
    'outward from the TV like a halo. The CRT screen shows green-on-black '
    'phosphor terminal text "benjamin@benterm ~" with a blinking green block '
    'cursor below. On the upper right corner of the TV bezel, a small glowing '
    'red rectangular pixel-art badge reading "BEN" in all caps. A tiny red power '
    'LED on the lower right of the bezel. Subtle scanline texture on the screen. '
    'Cinematic product render, deep contrast, no extra text or logos. Captured '
    'with Sony A7R IV at 50mm f/2.8, soft directional rim light, Wallpaper* '
    'magazine product editorial.'
)


def load_api_key() -> str:
    key = os.environ.get("KIE_AI_API_KEY", "").strip()
    if key:
        return key
    env_path = Path("E:/Rankenstein/Benjamin/.env")
    if env_path.exists():
        for line in env_path.read_text(encoding="utf-8").splitlines():
            if line.startswith("KIE_AI_API_KEY="):
                return line.split("=", 1)[1].strip().strip('"').strip("'")
    raise SystemExit("KIE_AI_API_KEY not found in env or .env file")


def http_post_json(url: str, body: dict, api_key: str) -> dict:
    req = urllib.request.Request(
        url,
        data=json.dumps(body).encode("utf-8"),
        headers={
            "Authorization": f"Bearer {api_key}",
            "Content-Type": "application/json",
            "Accept": "application/json",
            "User-Agent": USER_AGENT,
        },
        method="POST",
    )
    with urllib.request.urlopen(req, timeout=60) as resp:
        return json.loads(resp.read().decode("utf-8"))


def http_get_json(url: str, api_key: str) -> dict:
    req = urllib.request.Request(
        url,
        headers={
            "Authorization": f"Bearer {api_key}",
            "Accept": "application/json",
            "User-Agent": USER_AGENT,
        },
    )
    with urllib.request.urlopen(req, timeout=60) as resp:
        return json.loads(resp.read().decode("utf-8"))


def http_get_bytes(url: str) -> tuple[bytes, str]:
    req = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
    with urllib.request.urlopen(req, timeout=120) as resp:
        return resp.read(), resp.headers.get("Content-Type", "image/png")


def submit_task(api_key: str, prompt: str) -> str:
    payload = http_post_json(
        KIE_CREATE_URL,
        {"model": MODEL, "input": {"prompt": prompt, "nsfw_checker": False}},
        api_key,
    )
    code = payload.get("code")
    if code and code != 200:
        raise RuntimeError(f"createTask failed: {payload}")
    task_id = payload.get("data", {}).get("taskId")
    if not task_id:
        raise RuntimeError(f"createTask returned no taskId: {payload}")
    return task_id


def wait_for_task(api_key: str, task_id: str) -> str:
    deadline = time.time() + TIMEOUT_SECONDS
    last_state = None
    while time.time() < deadline:
        info = http_get_json(f"{KIE_RECORD_URL}?taskId={task_id}", api_key)
        if info.get("code") and info["code"] != 200:
            raise RuntimeError(f"recordInfo error: {info}")
        data = info.get("data") or {}
        state = (data.get("state") or "").lower()
        if state != last_state:
            print(f"  task {task_id}: {state}")
            last_state = state
        if state == "success":
            return extract_url(data.get("resultJson"))
        if state == "fail":
            raise RuntimeError(
                f"task failed: {data.get('failMsg') or data.get('failCode')}"
            )
        time.sleep(POLL_SECONDS)
    raise RuntimeError(f"task {task_id} timed out after {TIMEOUT_SECONDS}s")


def extract_url(result_json) -> str:
    if isinstance(result_json, str):
        try:
            result_json = json.loads(result_json)
        except json.JSONDecodeError:
            if result_json.startswith("http"):
                return result_json
            raise RuntimeError(f"unparseable resultJson: {result_json[:200]}")
    if isinstance(result_json, dict):
        for key in ("resultUrls", "urls"):
            urls = result_json.get(key)
            if isinstance(urls, list) and urls:
                return str(urls[0])
        for key in ("resultUrl", "url", "output_url", "image_url"):
            if result_json.get(key):
                return str(result_json[key])
    raise RuntimeError(f"no result url in {json.dumps(result_json)[:300]}")


def crop_resize(src: Path, dest: Path, width: int, height: int, fmt: str = "webp", quality: int = 86) -> None:
    dest.parent.mkdir(parents=True, exist_ok=True)
    cmd = [
        "magick",
        str(src),
        "-resize",
        f"{width}x{height}^",
        "-gravity",
        "center",
        "-extent",
        f"{width}x{height}",
    ]
    if fmt == "webp":
        cmd += ["-define", f"webp:quality={quality}", str(dest)]
    elif fmt == "jpg":
        cmd += ["-quality", str(quality), str(dest)]
    else:
        cmd += [str(dest)]
    subprocess.run(cmd, check=True)


def generate(api_key: str, prompt: str, label: str, dest: Path, width: int, height: int, fmt: str) -> None:
    print(f"\n[{label}] submitting task")
    task_id = submit_task(api_key, prompt)
    print(f"[{label}] task_id={task_id}")
    url = wait_for_task(api_key, task_id)
    print(f"[{label}] result url: {url}")
    image_bytes, content_type = http_get_bytes(url)
    suffix = ".png"
    if "webp" in content_type:
        suffix = ".webp"
    elif "jpeg" in content_type or "jpg" in content_type:
        suffix = ".jpg"
    TMP.mkdir(parents=True, exist_ok=True)
    raw_path = TMP / f"{label}-source{suffix}"
    raw_path.write_bytes(image_bytes)
    print(f"[{label}] saved source -> {raw_path}")
    crop_resize(raw_path, dest, width, height, fmt=fmt)
    print(f"[{label}] wrote final -> {dest} ({width}x{height} {fmt})")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--banner-only", action="store_true")
    parser.add_argument("--social-only", action="store_true")
    args = parser.parse_args()

    api_key = load_api_key()

    if not args.social_only:
        generate(
            api_key,
            BANNER_PROMPT,
            "banner",
            ASSETS / "banner.webp",
            width=1915,
            height=821,
            fmt="webp",
        )
    if not args.banner_only:
        generate(
            api_key,
            SOCIAL_PROMPT,
            "social",
            ASSETS / "github-social-preview.jpg",
            width=1280,
            height=640,
            fmt="jpg",
        )

    print("\nDone. Review the assets visually before committing.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
