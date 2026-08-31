<script lang="ts">
  export let progress: number = 0; // 0.0 to 1.0
  export let status: string = '';
</script>

<div class="progress-container">
  <div class="info-row">
    <span class="status-label">{status || 'Processing...'}</span>
    <span class="percentage">{Math.round(progress * 100)}%</span>
  </div>
  <div class="track">
    <div class="fill" style="width: {Math.min(Math.max(progress * 100, 0), 100)}%;">
      <div class="shimmer"></div>
    </div>
  </div>
</div>

<style>
  .progress-container {
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .info-row {
    display: flex;
    justify-content: space-between;
    font-size: 0.85rem;
    color: #94a3b8;
  }

  .status-label {
    font-weight: 500;
  }

  .percentage {
    font-family: 'JetBrains Mono', monospace;
    font-weight: 600;
    color: #38bdf8;
  }

  .track {
    width: 100%;
    height: 8px;
    background: #1e293b;
    border-radius: 9999px;
    overflow: hidden;
    position: relative;
  }

  .fill {
    height: 100%;
    background: linear-gradient(90deg, #0284c7, #38bdf8);
    border-radius: 9999px;
    transition: width 0.2s ease-out;
    position: relative;
    overflow: hidden;
  }

  .shimmer {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: linear-gradient(
      90deg,
      rgba(255, 255, 255, 0) 0%,
      rgba(255, 255, 255, 0.3) 50%,
      rgba(255, 255, 255, 0) 100%
    );
    animation: shimmer 1.5s infinite;
  }

  @keyframes shimmer {
    0% { transform: translateX(-100%); }
    100% { transform: translateX(100%); }
  }
</style>
