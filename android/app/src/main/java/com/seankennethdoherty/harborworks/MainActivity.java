package com.seankennethdoherty.harborworks;

import android.app.Activity;
import android.graphics.Typeface;
import android.os.Bundle;
import android.view.Gravity;
import android.view.View;
import android.widget.Button;
import android.widget.LinearLayout;
import android.widget.ScrollView;
import android.widget.TextView;

public final class MainActivity extends Activity {
    private static final String REPLAY_SUMMARY =
            "Deterministic replay: starter_rescue_skiff.json\n"
                    + "Duration: 10 seconds\n"
                    + "Ticks: 100\n"
                    + "Final position: (0.000, 0.000, 662.288)\n"
                    + "Final speed: 101.978 m/s\n"
                    + "Energy placeholder: 814614.875 J";

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        int outerPadding = dp(24);
        int gap = dp(14);

        ScrollView scrollView = new ScrollView(this);
        scrollView.setFillViewport(true);
        scrollView.setBackgroundColor(0xFFF7FAFC);

        LinearLayout root = new LinearLayout(this);
        root.setOrientation(LinearLayout.VERTICAL);
        root.setGravity(Gravity.CENTER_HORIZONTAL);
        root.setPadding(outerPadding, outerPadding, outerPadding, outerPadding);
        scrollView.addView(root, new ScrollView.LayoutParams(
                ScrollView.LayoutParams.MATCH_PARENT,
                ScrollView.LayoutParams.WRAP_CONTENT));

        TextView title = text("Harborworks", 34, Typeface.DEFAULT_BOLD, 0xFF0F172A);
        root.addView(title, matchWrap());

        TextView subtitle = text("Engineering sandbox prototype", 18, Typeface.DEFAULT, 0xFF334155);
        subtitle.setPadding(0, dp(6), 0, 0);
        root.addView(subtitle, matchWrap());

        TextView milestone = text("Milestone 0 foundation", 20, Typeface.DEFAULT_BOLD, 0xFF0F766E);
        milestone.setPadding(0, dp(24), 0, 0);
        root.addView(milestone, matchWrap());

        TextView features = text(
                "• typed IDs\n"
                        + "• blueprint validation\n"
                        + "• starter rescue skiff\n"
                        + "• deterministic replay",
                17,
                Typeface.DEFAULT,
                0xFF1E293B);
        features.setLineSpacing(dp(3), 1.0f);
        features.setPadding(0, gap, 0, 0);
        root.addView(features, matchWrap());

        TextView blueprint = text(
                "Bundled sample: starter rescue skiff blueprint with validated nodes, beams, panels, components, materials, and part references.",
                16,
                Typeface.DEFAULT,
                0xFF475569);
        blueprint.setPadding(0, dp(22), 0, 0);
        root.addView(blueprint, matchWrap());

        TextView replay = text(REPLAY_SUMMARY, 15, Typeface.MONOSPACE, 0xFF0F172A);
        replay.setVisibility(View.GONE);
        replay.setPadding(dp(14), dp(14), dp(14), dp(14));
        replay.setBackgroundColor(0xFFE2E8F0);

        Button button = new Button(this);
        button.setText("Show replay summary");
        button.setAllCaps(false);
        button.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View view) {
                boolean showing = replay.getVisibility() == View.VISIBLE;
                replay.setVisibility(showing ? View.GONE : View.VISIBLE);
                button.setText(showing ? "Show replay summary" : "Hide replay summary");
            }
        });
        LinearLayout.LayoutParams buttonParams = matchWrap();
        buttonParams.setMargins(0, dp(26), 0, gap);
        root.addView(button, buttonParams);
        root.addView(replay, matchWrap());

        TextView note = text(
                "Android shell only. Rust engine/editor integration is a future milestone.",
                14,
                Typeface.DEFAULT,
                0xFF64748B);
        note.setPadding(0, dp(24), 0, 0);
        root.addView(note, matchWrap());

        setContentView(scrollView);
    }

    private TextView text(String value, int sp, Typeface typeface, int color) {
        TextView view = new TextView(this);
        view.setText(value);
        view.setTextSize(sp);
        view.setTypeface(typeface);
        view.setTextColor(color);
        return view;
    }

    private LinearLayout.LayoutParams matchWrap() {
        return new LinearLayout.LayoutParams(
                LinearLayout.LayoutParams.MATCH_PARENT,
                LinearLayout.LayoutParams.WRAP_CONTENT);
    }

    private int dp(int value) {
        return Math.round(value * getResources().getDisplayMetrics().density);
    }
}
